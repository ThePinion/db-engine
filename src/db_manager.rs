use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

use crate::{
    db_class::{DbClass, DbClassIdentifier},
    db_field::{DbClassLinkMultiple, DbClassLinkSingle},
    db_relation::{DbRelation, DbRelationType},
};

pub struct DbManager {
    classes: HashMap<DbClassIdentifier, DbClass>,
}

impl DbManager {
    pub fn new() -> Self {
        DbManager {
            classes: HashMap::new(),
        }
    }

    pub fn add_class(&mut self, class: DbClass) -> DbClassIdentifier {
        let ident = class.ident.clone();
        self.classes.insert(ident.clone(), class);
        ident
    }

    pub fn add_relation(&mut self, relation: DbRelation) {
        self.add_one_side_of_relation(
            relation.left.clone(),
            relation.right.clone(),
            relation.prefetch,
        );
        self.add_one_side_of_relation(relation.right, relation.left, false);
    }
    fn add_one_side_of_relation(
        &mut self,
        left: DbRelationType,
        right: DbRelationType,
        prefetch: bool,
    ) {
        let left_iden = left.clone().to_relation_end().ident;
        let right_iden = right.to_relation_end().ident;
        let field = match &left {
            DbRelationType::Single(l) => {
                if prefetch {
                    DbClassLinkSingle::new_prefetch(&l.name, &right_iden)
                } else {
                    DbClassLinkSingle::new(&l.name, &right_iden)
                }
            }
            DbRelationType::Many(l) => {
                if prefetch {
                    DbClassLinkMultiple::new_prefetch(&l.name, &right_iden)
                } else {
                    DbClassLinkMultiple::new(&l.name, &right_iden)
                }
            }
        };
        self.classes
            .get_mut(&left_iden)
            .unwrap()
            .ref_add_field(field);
    }
}

impl DbManager {
    pub fn to_tokens(self) -> TokenStream {
        let struct_tokens = self
            .classes
            .into_values()
            .map(|c| {
                let struct_ = c.to_main_builder().to_tokens();
                let id_struct = c.to_id_builder().to_tokens();
                let create_struct = c.to_value_builder().to_tokens();
                let serializer_struct = c.to_serializer_builder().to_tokens();
                let impl_ = c.to_impl_tokens();
                let impl_from = c.to_impl_from_tokens();
                quote! {
                    #id_struct
                    #struct_
                    #create_struct
                    #serializer_struct
                    #impl_
                    #impl_from
                }
            })
            .collect::<Vec<_>>();
        quote! {
            use surrealdb::{Surreal, engine::remote::ws::Client};
            use serde::{Deserialize, Serialize, Deserializer, Serializer, ser::Error};
            use surrealdb::sql::Thing;
            use futures::future::join_all;

            #[derive(Debug, Deserialize)]
            struct Record {
                #[allow(dead_code)]
                id: Thing,
            }

            trait ClassHash{
                fn class_hash() -> String;
            }

            fn thing_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
            where
                D: Deserializer<'de>,
            {
                let original_value: Thing = Deserialize::deserialize(deserializer)?;
                Ok(original_value.id.to_string())
            }
            fn db_link_to_thing<S, T, U>(db_link: &DbLink<T, U>, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
                T: Into<Thing>,
                T: Clone
            {
                let DbLink::Existing(e) = db_link else {return Err(Error::custom("Unable to serialize DbLink::New"))};
                let thing: Thing = e.clone().into();
                thing.serialize(serializer)
            }
            fn db_link_to_vec_thing<S, T, U>(db_link: &DbLink<Vec<T>, U>, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
                T: Into<Thing>,
                T: Clone
            {
                let DbLink::Existing(e) = db_link else {return Err(Error::custom("Unable to serialize DbLink::New"))};
                let vec: Vec<Thing> = e.iter().map(|i| i.clone().into()).collect();
                vec.serialize(serializer)
            }

            #[derive(Debug, Serialize, Deserialize, Clone)]
            pub enum DbLink<S, T> {
                Existing(S),
                New(T)
            }

            #(#struct_tokens)*
        }
    }
}
