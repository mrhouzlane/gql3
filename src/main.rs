use reqwest;
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
struct QueryResponse {
    data: Option<Data>,
    errors: Option<Vec<Error>>,
}

#[derive(Debug, Deserialize)]
struct Data {
    launchedEscrows: Vec<LaunchedEscrow>,
}

#[derive(Debug, Deserialize)]
struct LaunchedEscrow {
    id: String,
    token: String,
    from: String,
    timestamp: String,
}

#[derive(Debug, Deserialize)]
struct Error {
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let query = r#"
        {
            launchedEscrows(first: 5) {
                id
                token
                from
                timestamp
            }
        }
    "#;

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.thegraph.com/subgraphs/name/humanprotocol/mumbai-v1")
        .json(&json!({
            "query": query,
        }))
        .send()
        .await?
        .json::<QueryResponse>()
        .await?;

    if let Some(errors) = res.errors {
        for error in errors {
            println!("Error: {}", error.message);
        }
    } else if let Some(data) = res.data {
        println!("{:#?}", data.launchedEscrows);
    } else {
        println!("Response did not contain data or errors");
    }

    Ok(())
}