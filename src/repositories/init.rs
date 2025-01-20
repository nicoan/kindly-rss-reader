//! This module inits the database connection pool and run migrations if needed

use std::fs::{read_dir, read_to_string};

use sqlite::ConnectionThreadSafe;

use crate::config::Config;

pub fn init_database(config: &Config) -> ConnectionThreadSafe {
    println!("{}", config.data_path);
    let connection: ConnectionThreadSafe =
        sqlite::Connection::open_thread_safe(format!("{}/database.db", config.data_path))
            .expect("there was an error opening a connection to the database");

    let migration_files = read_dir(format!("{}/migrations", config.static_data_path))
        .expect("unable to read migrations directory");

    for migration in migration_files {
        let migration = migration.unwrap();
        let migration = migration.path();
        let migration_content = read_to_string(&migration)
            .unwrap_or_else(|_| panic!("unable to read migration: {migration:?}"));
        connection
            .execute(&*migration_content)
            .unwrap_or_else(|_| panic!("unable to run migration: {migration:?}"));
    }

    connection
}
