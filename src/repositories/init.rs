//! This module inits the database connection pool and run migrations if needed

use std::{
    ffi::OsString,
    fs::{read_dir, read_to_string, DirEntry},
    path::Path,
};

use sqlite::ConnectionThreadSafe;

use crate::config::Config;

const MIGRATIONS_TABLE_FILE: &str = "migrations.sql";

fn execute_migration(connection: &ConnectionThreadSafe, migration_file: impl AsRef<Path>) {
    tracing::info!("migrating {:?}", migration_file.as_ref());
    let migration_content = read_to_string(migration_file).expect("unable to read migration file");

    connection
        .execute(&*migration_content)
        .expect("unable to run migration file");
}

pub fn init_database(config: &Config) -> ConnectionThreadSafe {
    let connection: ConnectionThreadSafe =
        sqlite::Connection::open_thread_safe(format!("{}/database.db", config.data_path))
            .expect("there was an error opening a connection to the database");

    // First we create (if needed) the migrations table and query it to filter already applied
    // migrations
    execute_migration(
        &connection,
        format!(
            "{}/migrations/{MIGRATIONS_TABLE_FILE}",
            config.static_data_path
        ),
    );

    let migrations_done: Vec<OsString> = connection
        .prepare("SELECT name FROM migrations")
        .expect("there was an error querying the migrations table")
        .into_iter()
        .map(|row| {
            let row = row.expect("unable to read row from migrations table");
            row.read::<&str, _>("name").into()
        })
        .collect();

    let migration_files: Vec<DirEntry> =
        read_dir(format!("{}/migrations", config.static_data_path))
            .expect("unable to read migrations directory")
            .collect::<Result<Vec<DirEntry>, _>>()
            .expect("unable to read migrations directory")
            .into_iter()
            .filter(|m| {
                !migrations_done.contains(&m.file_name()) && m.file_name() != MIGRATIONS_TABLE_FILE
            })
            .collect();

    for migration in migration_files {
        let migration_filename = migration.file_name();
        let migration_filename = migration_filename
            .to_str()
            .expect("unable to parse migration file name");

        let migration = migration.path();
        execute_migration(&connection, migration);

        let mut stmt = connection
            .prepare("INSERT INTO migrations (name) VALUES (?)")
            .expect("unable to prepare migration sentence");
        stmt.bind((1, migration_filename))
            .expect("unable to prepare migration sentence");

        stmt.next().expect("unable to process migration");
    }

    connection
}
