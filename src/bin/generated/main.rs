mod types;

use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use types::ValueUser;

use crate::types::{DbLink, ValuePet};

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db_username = std::env::var("DB_USERNAME").unwrap();
    let db_password = std::env::var("DB_PASSWORD").unwrap();
    let db_ws = std::env::var("DB_WS").unwrap();

    // Connect to the server
    let db = Surreal::new::<Ws>(db_ws).await?;

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: db_username.as_str(),
        password: db_password.as_str(),
    })
    .await
    .unwrap();

    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await?;

    let user = ValueUser {
        age: 20,
        email: "test@test.gmail".to_string(),
        name: "Debil".to_string(),
    }
    .db_create(&db)
    .await?;
    dbg!(&user);

    let pet = ValuePet {
        name: "Azor".to_string(),
        owner: DbLink::Existing(user),
        doctor: DbLink::New(vec![ValueUser {
            age: 20,
            email: "test@test.gmail".to_string(),
            name: "Konstantyn".to_string(),
        }]),
        caretaker: DbLink::New(vec![
            ValueUser {
                age: 20,
                email: "test@test.gmail".to_string(),
                name: "Konstantyn".to_string(),
            },
            ValueUser {
                age: 20,
                email: "test@test.gmail".to_string(),
                name: "Konstantyn".to_string(),
            },
            ValueUser {
                age: 20,
                email: "test@test.gmail".to_string(),
                name: "Konstantyn".to_string(),
            }
        ]),
    }
    .db_create_get(&db)
    .await?;
    dbg!(pet);

    Ok(())
}
