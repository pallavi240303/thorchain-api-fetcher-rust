use async_trait::async_trait;
use std::error::Error;
use crate::models::{DepthInterval, EarningInterval, RunePoolInterval, SwapsInterval};


#[async_trait]
pub trait Database: Send + Sync {
    async fn store_depth_intervals(&self , interval: &DepthInterval) -> Result<() , Box<dyn Error>>;
    async fn store_swaps_intervals(&self , interval: &SwapsInterval) -> Result<() , Box<dyn Error>>;
    async fn store_earnings_intervals(&self , interval: &EarningInterval) -> Result<() , Box<dyn Error>>;
    async fn store_runepool_intervals(&self , interval: &RunePoolInterval) -> Result<() , Box<dyn Error>>;
}