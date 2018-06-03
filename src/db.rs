use diesel::pg::PgConnection;
use diesel::Connection;
use failure::{Error, ResultExt};
use model::salt::Salt;
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

pub fn salt_component() -> Result<String, Error> {
    use diesel::prelude::*;
    use schema::salts::dsl::*;

    let connection = establish_connection();

    let result = salts
        .first::<Salt>(&connection)
        .with_context(|e| format!("Failed to read salt from database\n => {}", e))?;

    Ok(result.salt.to_string())
}
