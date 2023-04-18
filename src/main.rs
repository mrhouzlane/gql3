#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use graphql_client::{GraphQLQuery, Response};
use serde_derive::{Deserialize, Serialize};
use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result as AnyhowResult;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
struct Escrows(String);


#[derive(Serialize, Deserialize, Debug)]
struct LaunchedEscrow {
    id: Escrows,
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/schema.graphql",
    query_path = "src/query.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub struct MyQuery;

async fn perform_my_query(variables: my_query::Variables,
) -> Result<Response<my_query::ResponseData>, Error> {
    let request_body = MyQuery::build_query(variables);
    let client = reqwest::Client::new();
    let res = client.post("https://api.thegraph.com/subgraphs/name/humanprotocol/mumbai-v1")
    .json(&request_body)
    .send()
    .await?;
    let response_body: Response<my_query::ResponseData> =       res.json().await?;
    Ok(response_body)
}

#[tokio::main]
async fn main() -> AnyhowResult<(), Error> {
    let variables = my_query::Variables;
    let escrows_list = perform_my_query(variables)
        .await?
        .data
        .ok_or(anyhow!("Query failed"))?
        .launched_escrows;
    let mut escrows: Vec<Escrows> = Vec::new();
    for raw_escrow in escrows_list {
        let escrow_value = serde_json::to_value(raw_escrow).expect("Failed converting raw escrow to json value");
        let escrow: LaunchedEscrow = serde_json::from_value(escrow_value)
        .expect("Failed converting json value to escrow object");
        escrows.push(escrow.id);
    }
    println!("{:?}", escrows);
    Ok(())
}