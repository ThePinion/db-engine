use crate::db_class::DbClassIdentifier;

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct DbClassSimpleField {
    pub name: String,
    pub type_: String,
}

impl DbClassSimpleField {
    pub fn new(name: &str, type_: &str) -> DbClassField {
        DbClassSimpleField {
            name: name.to_string(),
            type_: type_.to_string(),
        }
        .into()
    }
}
#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct DbClassLinkMultiple {
    pub name: String,
    pub ident: DbClassIdentifier,
    pub prefetch: bool,
}

impl DbClassLinkMultiple {
    pub fn new(name: impl Into<String>, ident: &DbClassIdentifier) -> DbClassField {
        DbClassLinkMultiple {
            name: name.into(),
            ident: ident.clone(),
            prefetch: false,
        }
        .into()
    }
    pub fn new_prefetch(name: impl Into<String>, ident: &DbClassIdentifier) -> DbClassField {
        DbClassLinkMultiple {
            name: name.into(),
            ident: ident.clone(),
            prefetch: true,
        }
        .into()
    }
}
#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct DbClassLinkSingle {
    pub name: String,
    pub ident: DbClassIdentifier,
    pub prefetch: bool,
}

impl DbClassLinkSingle {
    pub fn new(name: impl Into<String>, ident: &DbClassIdentifier) -> DbClassField {
        DbClassLinkSingle {
            name: name.into(),
            ident: ident.clone(),
            prefetch: false,
        }
        .into()
    }
    pub fn new_prefetch(name: impl Into<String>, ident: &DbClassIdentifier) -> DbClassField {
        DbClassLinkSingle {
            name: name.into(),
            ident: ident.clone(),
            prefetch: true,
        }
        .into()
    }
}

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub enum DbClassField {
    Simple(DbClassSimpleField),
    LinkSingle(DbClassLinkSingle),
    LinkMultiple(DbClassLinkMultiple),
}

impl From<DbClassSimpleField> for DbClassField {
    fn from(value: DbClassSimpleField) -> Self {
        DbClassField::Simple(value)
    }
}

impl From<DbClassLinkSingle> for DbClassField {
    fn from(value: DbClassLinkSingle) -> Self {
        DbClassField::LinkSingle(value)
    }
}

impl From<DbClassLinkMultiple> for DbClassField {
    fn from(value: DbClassLinkMultiple) -> Self {
        DbClassField::LinkMultiple(value)
    }
}
