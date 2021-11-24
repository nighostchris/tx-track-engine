use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct GetBlockHeightResponse {
    jsonrpc: String,
    result: i64,
    id: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let solana_node = "https://api.mainnet-beta.solana.com";

    let client = reqwest::Client::new();

    let res = client.post(solana_node)
        .header("Content-Type", "application/json")
        .json(&HashMap::from([
            ("jsonrpc", "2.0"),
            ("id", "1"),
            ("method", "getBlockHeight"),
        ]))
        .send()
        .await?;

    let latest_block = res
        .json::<GetBlockHeightResponse>()
        .await?
        .result;

    println!("{:?}", latest_block);
    Ok(())
}
