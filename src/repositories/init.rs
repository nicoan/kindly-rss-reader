//! This module inits the database connection pool and run migrations if needed

use std::fs::{read_dir, read_to_string};

use sqlite::ConnectionThreadSafe;

pub fn init_database() -> ConnectionThreadSafe {
    let connection: ConnectionThreadSafe = sqlite::Connection::open_thread_safe("./database.db")
        .expect("there was an error opening a connection to the database");

    let migration_files = read_dir("./migrations").expect("unable to read migrations directory");
    for migration in migration_files {
        let migration = migration.unwrap();
        let migration = migration.path();
        let migration_content = read_to_string(&migration)
            .unwrap_or_else(|_| panic!("unable to read migration: {migration:?}"));
        connection
            .execute(&*migration_content)
            .unwrap_or_else(|_| panic!("unable to run migration: {migration:?}"));
    }

    /*
    connection
            .execute(
                r#"
           INSERT INTO feed (id, title, url, last_updated) VALUES ('5ece7b14-0e2b-45c6-b033-4c4e2d17d5cf', 'Nico Antinori', 'https://nicoan.net/index.xml', '1970-01-01T00:00:00Z');
        "#,
            )
            .unwrap();
    */
    connection
}
