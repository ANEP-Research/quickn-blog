use diesel::prelude::*;
use diesel::pg::PgConnection;

use crate::CONFIG;

pub fn establish_connection() -> PgConnection {
    let config = CONFIG.clone();
    PgConnection::establish(&config.general().database_url())
        .expect(&format!("Error connecting to {}", config.general().database_url()))
}