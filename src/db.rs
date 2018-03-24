use diesel::Connection;
use diesel::pg::PgConnection;
use failure::{Error, ResultExt};
use std::env;

pub fn establish_connection() -> PgConnection {
    let connect = || -> Result<PgConnection, Error> {
        let env_var = env::var("DATABASE_URL")
            .with_context(|e| format!("Failed to parse env variable DATABASE_URL\n => {}", e))?;

        let connection = PgConnection::establish(&env_var[..])
            .with_context(|e| format!("Failed to connect to database\n => {}", e))?;

        Ok(connection)
    };

    connect().unwrap_or_else(|e| {
        println!("{}", e);
        panic!();
    })
}

pub fn salt_component() -> Result<&str, Error> {
    use diesel::prelude::*;
    use schema::salt::dsl::*;

    let connection = establish_connection();

    let salt = salts
        .first::<Salt>(&connection)
        .with_context(|e| format!("Failed to read salt from database\n => {}", e))?;

    Ok(salt)
}
