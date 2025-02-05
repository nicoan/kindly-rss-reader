pub mod error;
pub mod feed;
pub mod feed_content;
mod init;
pub mod persisted_config;

pub use error::RepositoryError;
pub use init::init_database;

#[macro_export]
macro_rules! transaction {
    ($self: ident, $transaction: expr) => {{
        ($self).connection.execute("BEGIN;")?;

        let result = (|| $transaction)();

        match result {
            Ok(result) => {
                $self.connection.execute("COMMIT;")?;
                Ok(result)
            }
            Err(e) => {
                $self.connection.execute("ROLLBACK;")?;
                Err(e)
            }
        }
    }};
}
