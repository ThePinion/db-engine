use std::fs::File;
use std::io::Write;

use db_engine::{
    db_class::DbClass, db_field::DbClassLinkMultiple as LnM, db_field::DbClassLinkSingle as LnS,
    db_field::DbClassSimpleField as SF, db_manager::DbManager, db_relation::DbRelBuilder,
};

fn main() {
    let mut manager = DbManager::new();
    let user = manager.add_class(
        DbClass::with_name("User")
            .add_field(SF::new("name", "String"))
            .add_field(SF::new("email", "String"))
            .add_field(SF::new("age", "u16")),
    );
    let pet = manager.add_class(
        DbClass::with_name("Pet")
            .add_field(SF::new("name", "String"))
    );
    manager.add_relation(
        DbRelBuilder::new(&pet)
            .has_single("owner", &user)
            .which_have_many("pets")
            .to_prefetch_relation(),
    );
    manager.add_relation(
        DbRelBuilder::new(&pet)
            .has_many("doctors", &user)
            .which_have_many("patients")
            .to_relation(),
    );
    manager.add_relation(
        DbRelBuilder::new(&pet)
            .has_many("caretakers", &user)
            .which_have_many("carees")
            .to_relation(),
    );
    let tokens = manager.to_tokens();
    let code = prettyplease::unparse(&syn::parse2(tokens).unwrap());
    let path = "src/bin/generated/types.rs";
    File::create(path)
        .unwrap()
        .write_all(code.as_bytes())
        .unwrap();
    println!("Written to file: {}", path);
}
