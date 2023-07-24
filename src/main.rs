pub mod lib;
pub mod schema;

use std::{net::Ipv4Addr, path::Path, env};
use diesel::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use rocket::{routes, Config};
use lib::controllers::users::{delete_user, get_user_by_id, get_users, update_user};
use lib::controllers::auth::{signin, signup};


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
pub struct MyAppContext {
    pub pg_connection: PgConnection,
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let config = Config {
        port: 8080,
        address: Ipv4Addr::new(0, 0, 0, 0).into(),
        temp_dir: "/tmp/config-example".into(),
        ..Config::debug_default()
    };

    let rocket = rocket::custom(&config).ignite().await?;
    assert_eq!(rocket.config().temp_dir.relative(), Path::new("/tmp/config-example"));

    rocket::build().configure(rocket.config()).mount("/users", routes![delete_user, get_user_by_id, get_users, update_user]).mount("/auth", routes![signin, signup])
    .ignite().await?
    .launch().await?;

    Ok(())
}
