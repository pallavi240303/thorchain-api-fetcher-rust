use api::api_fetcher::{fetch_depth_data, fetch_earnings_data, fetch_runepool_data, fetch_swaps_data, IntervalParams};

mod models;
mod api;
#[tokio::main]
async fn main() {
    let params = IntervalParams {
        from: 1726758000,
        count: 1,
        interval: "hour".to_string(),
    };

    // Fetch depth data
    match fetch_depth_data(&params, "BTC.BTC").await {
        Ok(depth_data) => println!("Depth intervals fetched: {:#?}", depth_data),
        Err(e) => eprintln!("Failed to fetch depth data: {}", e),
    }

    // Fetch swaps data
    match fetch_swaps_data(&params).await {
        Ok(depth_data) => println!("Swaps intervals fetched: {:#?}", depth_data),
        Err(e) => eprintln!("Failed to fetch swap data: {}", e),
    }

    // Fetch earnings data
    match fetch_earnings_data(&params).await {
        Ok(earnings_data) => println!("Earnings intervals fetched: {:#?}", earnings_data),
        Err(e) => eprintln!("Failed to fetch earnings data: {}", e),
    }

    // Fetch rune pool data
    match fetch_runepool_data(&params).await {
        Ok(runepool_data) => println!("Rune pool intervals fetched: {:#?}", runepool_data),
        Err(e) => eprintln!("Failed to fetch rune pool data: {}", e),
    }
}
