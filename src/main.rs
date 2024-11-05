use std::env;

use api::api_fetcher::{fetch_depth_data, fetch_earnings_data, fetch_runepool_data, fetch_swaps_data, IntervalParams};
use dotenv::dotenv;
use repositories::postgres::postgres_db::PostgresDb;
use services::db_traits::Database; 
mod models;
mod api;
mod services;
mod repositories;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let postgres_url = env::var("POSTGRES_URL").expect("POSTGRES_URL must be set");
    let psql_db = match PostgresDb::new(&postgres_url).await {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Failed to connect to the database: {}", e);
            return;
        }
    };

    println!("DATABASE CONNECTED SUCCESSFULLY!");

    let params = IntervalParams {
        from: 1726758000,
        count: 1,
        interval: "hour".to_string(),
    };

    // Fetch depth data
    let depth_data = match fetch_depth_data(&params, "BTC.BTC").await {
        Ok(depth_data) => depth_data,
        Err(e) => {eprintln!("Failed to fetch depth data: {}", e); return;},
    };

    for interval in depth_data {
        if let Err(e) = psql_db.store_depth_intervals(&interval).await {
            eprintln!("Failed to store depth interval: {}", e);
        }
    }
    println!("DEPTH DATA INSERTED SUCCESSFULLY!");

    // Fetch swaps data
    let swaps_data = match fetch_swaps_data(&params).await {
        Ok(swaps_data) => swaps_data,
        Err(e) => { eprintln!("Failed to fetch swap data: {}", e); return; },
    };

    for interval in swaps_data {
        if let Err(e) = psql_db.store_swaps_intervals(&interval).await {
            eprintln!("Failed to store swap interval: {}", e);
        }
    }
    println!("SWAP DATA INSERTED SUCCESSFULLY!");

    // Fetch earnings data
    let earnings_data = match fetch_earnings_data(&params).await {
        Ok(earnings_data) => earnings_data,
        Err(e) => { eprintln!("Failed to fetch earnings data: {}", e); return; },
    };

    for interval in earnings_data {
        if let Err(e) = psql_db.store_earnings_intervals(&interval).await {
            eprintln!("Failed to store earnings interval: {}", e);
        }
    }
    println!("EARNING DATA INSERTED SUCCESSFULLY!");

    // Fetch rune pool data
    let runepool_data = match fetch_runepool_data(&params).await {
        Ok(runepool_data) => runepool_data,
        Err(e) => { eprintln!("Failed to fetch rune pool data: {}", e); return; },
    };

    for interval in runepool_data {
        if let Err(e) = psql_db.store_runepool_intervals(&interval).await {
            eprintln!("Failed to store rune pool interval: {}", e);
        }
    }
    println!("RUNEPOOL DATA INSERTED SUCCESSFULLY!");
}
