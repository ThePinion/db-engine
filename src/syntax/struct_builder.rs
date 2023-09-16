use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, ToTokens};

use crate::db_class::DbClass;

pub struct Field {
    name: String,
    field_type: String,
    decorators: Vec<String>,
}

impl Field {
    pub fn with_decorators(
        name: impl Into<String>,
        field_type: impl Into<String>,
        decorators: Vec<impl Into<String>>,
    ) -> Self {
        Field {
            name: name.into(),
            field_type: field_type.into(),
            decorators: decorators.into_iter().map(|d| d.into()).collect(),
        }
    }
    pub fn new(name: impl Into<String>, field_type: impl Into<String>) -> Self {
        Field::with_decorators(name, field_type, vec![] as Vec<String>)
    }
    pub fn to_tokens(&self) -> TokenStream {
        let name_iden = format_ident!("{}", self.name);
        // let type_iden = format_ident!("{}", self.field_type);
        let type_iden: TokenStream = syn::parse_str::<TokenStream>(&self.field_type)
            .unwrap()
            .into_token_stream();
        let decorators = &self
            .decorators
            .iter()
            .map(|d| d.parse::<TokenStream>().unwrap())
            .collect::<Vec<_>>();
        quote! {
            #(#decorators)*
            pub #name_iden: #type_iden
        }
    }
}

pub struct StructSyntaxBuilder {
    name: String,
    fields: Vec<Field>,
}

impl StructSyntaxBuilder {
    pub fn new(name: &str) -> Self {
        StructSyntaxBuilder {
            name: name.to_string(),
            fields: Vec::new(),
        }
    }

    pub fn add_field(&mut self, field: Field) -> &mut Self {
        self.fields.push(field);
        self
    }

    fn name_iden(&self) -> Ident {
        syn::Ident::new(&self.name, Span::call_site())
    }

    pub fn to_tokens(&self) -> TokenStream {
        let name_iden = self.name_iden();
        let field_defs: Vec<_> = self.fields.iter().map(Field::to_tokens).collect();

        quote! {
            #[derive(Debug, Serialize, Deserialize, Clone)]
            pub struct #name_iden {
                #(#field_defs,)*
            }
        }
    }
}

impl DbClass {
    pub fn to_main_builder(&self) -> StructSyntaxBuilder {
        let mut builder = self.id_builder(&self.ident.name);
        builder = self.add_simple_fields(builder);
        builder = self.add_link_single_fields(builder);
        builder = self.add_link_multiple_fields(builder);
        builder
    }

    pub fn to_id_builder(&self) -> StructSyntaxBuilder {
        self.id_builder(&self.ident.id_struct_name())
    }
    pub fn to_value_builder(&self) -> StructSyntaxBuilder {
        let mut builder = StructSyntaxBuilder::new(&self.ident.value_struct_name());
        builder = self.add_simple_fields(builder);
        builder = self.add_link_single_fields_value(builder);
        builder = self.add_link_multiple_fields_value(builder);
        builder
    }
    pub fn to_serializer_builder(&self) -> StructSyntaxBuilder {
        let mut builder = StructSyntaxBuilder::new(&self.ident.serializer_struct_name());
        builder = self.add_simple_fields(builder);
        builder = self.add_link_single_fields_serializer(builder);
        builder = self.add_link_multiple_fields_serializer(builder);
        builder
    }

    fn id_builder(&self, name: &str) -> StructSyntaxBuilder {
        let mut a = StructSyntaxBuilder::new(name);
        a.add_field(Field::with_decorators(
            "id",
            "String",
            vec!["#[serde(deserialize_with = \"thing_to_string\")]"],
        ));
        a
    }
    fn add_simple_fields(&self, mut builder: StructSyntaxBuilder) -> StructSyntaxBuilder {
        for f in self.simple_fields() {
            builder.add_field(Field::new(&f.name, &f.type_));
        }
        builder
    }
    fn add_link_single_fields(&self, mut builder: StructSyntaxBuilder) -> StructSyntaxBuilder {
        for f in self.link_single_fields() {
            builder.add_field(Field::new(
                &f.name,
                if f.prefetch {
                    f.ident.name
                } else {
                    f.ident.id_struct_name()
                },
            ));
        }
        builder
    }
    fn add_link_single_fields_value(
        &self,
        mut builder: StructSyntaxBuilder,
    ) -> StructSyntaxBuilder {
        for f in self.link_single_fields() {
            builder.add_field(Field::with_decorators(
                &f.name,
                if f.prefetch {
                    format!(
                        "DbLink<{}, {}>",
                        f.ident.id_struct_name(),
                        f.ident.value_struct_name()
                    )
                } else {f.ident.id_struct_name()}
                ,
                vec!["#[serde(serialize_with = \"db_link_to_thing\")]"],
            ));
        }
        builder
    }
    fn add_link_single_fields_serializer(
        &self,
        mut builder: StructSyntaxBuilder,
    ) -> StructSyntaxBuilder {
        for f in self.link_single_fields() {
            builder.add_field(Field::new(&f.name, "Thing"));
        }
        builder
    }
    fn add_link_multiple_fields(&self, mut builder: StructSyntaxBuilder) -> StructSyntaxBuilder {
        for f in self.link_multiple_fields() {
            builder.add_field(Field::new(
                &f.name,
                format!(
                    "Vec<{}> ",
                    if f.prefetch {
                        f.ident.name
                    } else {
                        f.ident.id_struct_name()
                    }
                ),
            ));
        }
        builder
    }
    fn add_link_multiple_fields_value(
        &self,
        mut builder: StructSyntaxBuilder,
    ) -> StructSyntaxBuilder {
        for f in self.link_multiple_fields() {
            builder.add_field(Field::with_decorators(
                &f.name,
                if f.prefetch {
                    format!(
                        "DbLink<Vec<{}>, Vec<{}>>",
                        f.ident.id_struct_name(),
                        f.ident.value_struct_name()
                    )
                } else {format!("Vec<{}>", f.ident.id_struct_name())},
                vec!["#[serde(serialize_with = \"db_link_to_vec_thing\")]"],
            ));
        }
        builder
    }
    fn add_link_multiple_fields_serializer(
        &self,
        mut builder: StructSyntaxBuilder,
    ) -> StructSyntaxBuilder {
        for f in self.link_multiple_fields() {
            builder.add_field(Field::new(&f.name, "Vec<Thing>"));
        }
        builder
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_code() {
        let tokens = StructSyntaxBuilder::new("Person")
            .add_field(Field::new("name", "String"))
            .add_field(Field::new("age", "i32"))
            .to_tokens();
        let code = prettyplease::unparse(&syn::parse2(tokens).unwrap());
        assert_eq!(
            code,
            "struct Person {\n    name: String,\n    age: i32,\n}\n"
        );
    }
}
