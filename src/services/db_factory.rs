use std::error::Error;
use crate::repositories::postgres::postgres_db::PostgresDb;
use super::db_traits::Database;
#[derive(Debug)]
pub enum DbType {
    Postgres(String),
    MongoDb(String,String),
    RocksDb(String),
    // LevelDb(String),
    // SurrealDb(String)
}

pub fn match_database_type(db_type: &str, args: &[String]) -> Result<DbType, Box<dyn Error>> {
    match db_type {
        "postgres" if args.len() > 2 => {
            let conn_str = &args[2]; 
            Ok(DbType::Postgres(conn_str.clone()))
        },
        "mongodb" if args.len() > 3 => {
            let uri = &args[2]; 
            let db_name = &args[3]; 
            Ok(DbType::MongoDb(uri.clone(), db_name.clone()))
        },
        "rocksdb" if args.len() > 2 => {
            let path = &args[2]; 
            Ok(DbType::RocksDb(path.clone()))
        },
        _ => Err("Unsupported or insufficient arguments for the specified database type".into()),
    }
}

pub struct DatabaseFactory;

impl DatabaseFactory {
    pub async fn create(db: DbType) -> Result<Box<dyn Database> , Box<dyn Error>> {
        match db {
            DbType::Postgres(conn) => {
                let postgres_db = PostgresDb::new(&conn).await?;
                Ok(Box::new(postgres_db))
            }
            DbType::MongoDb(uri, db_name) => {
                // Placeholder for MongoDB implementation
                // let mongo_db = MongoDb::new(&uri, &db_name).await?;
                // Ok(Box::new(mongo_db))
                Err("MongoDB is not implemented".into())
            },
            DbType::RocksDb(path) => {
                // Placeholder for RocksDB implementation
                // let rocks_db = RocksDb::new(&path)?;
                // Ok(Box::new(rocks_db))
                Err("RocksDB is not implemented".into())
            },
        }
        
    }
}