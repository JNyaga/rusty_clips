use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use sqlx::Sqlite;
use std::str::FromStr;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum DataError {
    #[error("database error:{0}")]
    Database(#[from] sqlx::Error),
}

pub type AppDatabase = Database<Sqlite>;
pub type DatabasePool = sqlx::sqlite::SqlitePool;
pub type Transaction<'t> = sqlx::Transaction<'t, Sqlite>;
pub type AppDatabaseRow = sqlx::sqlite::SqliteRow;
pub type AppQueryResult = sqlx::sqlite::SqliteQueryResult;

//this struct is used to hold the database connection pool
pub struct Database<D: sqlx::Database>(sqlx::Pool<D>);

impl Database<Sqlite> {
    pub async fn new(connection_str: &str) -> Self {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect(connection_str)
            .await;
        match pool {
            Ok(pool) => Self(pool),
            Err(e) => {
                eprintln!("{}\n", e);
                eprintln!("If the database has not been created run: \n $sqlx database setup\n");
                panic!("Database connection error");
            }
        }
    }
    pub fn get_pool(&self) -> &DatabasePool {
        &self.0
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Display, From)]
pub struct Dbid(Uuid);

impl Dbid {
    pub fn new() -> Dbid {
        // Uuid::new_v4().into() is the same as Dbid(Uuid::new_v4())
        Uuid::new_v4().into()
    }

    pub fn nil() -> Dbid {
        //this helps to create a new instance of the struct with a nil value
        //same as Dbid(Uuid::nil())
        Self(Uuid::nil())
    }
}

impl Default for Dbid {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for Dbid {
    type Err = uuid::Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        //the ? operator is used to propagate errors and convert them into the error type of the function which is ClipError::Id
        Ok(Dbid(Uuid::parse_str(id)?))
    }
}
