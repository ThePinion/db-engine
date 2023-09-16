use crate::db_class::DbClassIdentifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DbRelation {
    pub left: DbRelationType,
    pub right: DbRelationType,
    pub prefetch: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DbRelationType {
    Single(DbRelationEnd),
    Many(DbRelationEnd),
}

impl DbRelationType {
    pub fn to_relation_end(self) -> DbRelationEnd {
        match self {
            DbRelationType::Single(e) => e,
            DbRelationType::Many(e) => e,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DbRelationEnd {
    pub ident: DbClassIdentifier,
    pub name: String,
}

pub struct DbRelBuilder(DbClassIdentifier);
pub struct DbRelBuilderB(DbClassIdentifier, String, bool, DbClassIdentifier);
pub struct DbRelBuilderC(
    DbClassIdentifier,
    String,
    bool,
    DbClassIdentifier,
    String,
    bool,
);

impl DbRelBuilder {
    pub fn new(ident: &DbClassIdentifier) -> Self {
        DbRelBuilder(ident.clone())
    }
    pub fn has_many(self, name: impl Into<String>, ident: &DbClassIdentifier) -> DbRelBuilderB {
        DbRelBuilderB(self.0, name.into(), true, ident.clone())
    }
    pub fn has_single(self, name: impl Into<String>, ident: &DbClassIdentifier) -> DbRelBuilderB {
        DbRelBuilderB(self.0, name.into(), false, ident.clone())
    }
}

impl DbRelBuilderB {
    pub fn which_have_many(self, name: impl Into<String>) -> DbRelBuilderC {
        DbRelBuilderC(self.0, self.1, self.2, self.3, name.into(), true)
    }
    pub fn which_have_single(self, name: impl Into<String>) -> DbRelBuilderC {
        DbRelBuilderC(self.0, self.1, self.2, self.3, name.into(), false)
    }
}
impl DbRelBuilderC {
    pub fn to_relation(self) -> DbRelation {
        self.get_relation(false)
    }
    pub fn to_prefetch_relation(self) -> DbRelation {
        self.get_relation(true)
    }
    fn get_relation(self, prefetch: bool) -> DbRelation {
        let left_inner = DbRelationEnd {
            ident: self.0,
            name: self.1,
        };
        let left = if self.2 {
            DbRelationType::Many(left_inner)
        } else {
            DbRelationType::Single(left_inner)
        };
        let right_inner = DbRelationEnd {
            ident: self.3,
            name: self.4,
        };
        let right = if self.5 {
            DbRelationType::Many(right_inner)
        } else {
            DbRelationType::Single(right_inner)
        };
        DbRelation {
            left,
            right,
            prefetch,
        }
    }
}
