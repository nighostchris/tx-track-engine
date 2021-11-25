use serde::{Serialize, Deserialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct GetBlockHeightResponse {
    jsonrpc: String,
    result: i64,
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct SolanaTransactionInstruction {
    accounts: Vec<i64>,
    data: String,
    program_id_index: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct SolanaTransactionHeader {
    num_readonly_signed_accounts: Option<i64>,
    num_readonly_unsigned_accounts: Option<i64>,
    num_required_signatures: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct SolanaTransactionMessage {
    account_keys: Option<Vec<String>>,
    header: SolanaTransactionHeader,
    instructions: Vec<SolanaTransactionInstruction>,
    recent_blockhash: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct SolanaTransaction {
    message: SolanaTransactionMessage,
    signatures: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct SolanaTransactionInnerInstructionDetails {
    index: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct SolanaTransactionMetaError {

}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct SolanaTransactionMeta {
    fee: i64,
    inner_instructions: Option<Vec<SolanaTransactionInstruction>>,
    log_messages: Option<Vec<String>>,
    post_balances: Option<Vec<i64>>,
    post_token_balances: Option<Vec<i64>>,
    pre_balances: Option<Vec<i64>>,
    pre_token_balances: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct SolanaTransactionDetails {
    meta: SolanaTransactionMeta,
    transaction: SolanaTransaction,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct SolanaBlock {
    block_height: Option<i64>,
    block_time: Option<i64>,
    blockhash: String,
    parent_slot: Option<u64>,
    previous_blockhash: Option<String>,
    transactions: Vec<SolanaTransactionDetails>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GetBlockResponse {
    jsonrpc: String,
    result: SolanaBlock,
    id: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let solana_node = "https://api.mainnet-beta.solana.com";

    let client = reqwest::Client::new();

    // let get_block_height_request = json!({
    //     "jsonrpc": "2.0",
    //     "id": "1",
    //     "method": "getBlockHeight",
    // });

    // let res = client.post(solana_node)
    //     .header("Content-Type", "application/json")
    //     .json(&get_block_height_request)
    //     .send()
    //     .await?;

    // let latest_block = res
    //     .json::<GetBlockHeightResponse>()
    //     .await?
    //     .result;

    let get_block_body_request = json!({
        "jsonrpc": "2.0",
        "id": "1",
        "method": "getBlock",
        "params": [97797951],
    });

    let res = client.post(solana_node)
        .header("Content-Type", "application/json")
        .json(&get_block_body_request)
        .send()
        .await?;

    let block = res
        .json::<GetBlockResponse>()
        .await?;
    

    // println!("{:?}", latest_block);
    println!("{:?}", block.result.blockhash);

    Ok(())
}
