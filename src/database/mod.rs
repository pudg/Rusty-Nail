use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use diesel::prelude::*;
use std::env;
use r2d2;

use crate::errors::CustomError;

/// Defines alias for a maintained set of open connections to a database,
/// handling them out for repeated use.
type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Defines a type to a smart pointer wrapping a connection.
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

/// Defines a type for the backend being used (PostgreSQL).
pub type DB = diesel::pg::Pg;

/// Read migrations at compile time.
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();


pub fn run_migrations(connection: &mut impl MigrationHarness<DB>) -> Result<(), CustomError> {
    connection.run_pending_migrations(MIGRATIONS).expect("Unable to run PostgreSQL migrations.");
    Ok(())
}

lazy_static! {
    static ref POOL: Pool = {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        Pool::new(manager).expect("Failed to create database pool.")
    };
}

pub fn initialize() {
    lazy_static::initialize(&POOL);
    let mut connection = create_connection().expect("Failed to get database connection.");
    run_migrations(&mut connection).unwrap();
}

/// Returns active connection to PostgreSQL database.
pub fn create_connection() -> Result<DbConnection, CustomError> {
    POOL.get()
    .map_err(|err| CustomError::new(500, format!("Failed to create connection to database: {}", err)))
}