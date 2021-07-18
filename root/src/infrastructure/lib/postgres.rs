use crate::config::setting::{get_config, Config};
use diesel::{pg::PgConnection, Connection};

pub fn establish_connection() -> PgConnection {
    let config: Config = get_config();

    let database_url = config.database_url;

    let connection: PgConnection = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    connection
}
