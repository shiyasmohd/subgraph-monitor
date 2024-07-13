use serde::{Deserialize, Serialize};
use std::cmp::min;

#[derive(Deserialize, Debug)]
struct Url {
    url: String,
}
#[derive(Deserialize, Debug)]
struct Indexer {
    indexer: Url,
}

#[derive(Deserialize, Debug)]
struct Allocations {
    allocations: Vec<Indexer>,
}
#[derive(Deserialize, Debug)]
struct Response<T> {
    data: T,
}

#[derive(Deserialize, Debug)]
struct Chains {
    chains: Vec<ChainIndexingStatus>,
}

#[derive(Deserialize, Debug)]
struct ChainIndexingStatus {
    chainHeadBlock: Block,
    latestBlock: Block,
}

#[derive(Deserialize, Debug)]
struct IndexingStatus {
    indexingStatuses: Vec<Chains>,
}
#[derive(Deserialize, Debug)]
struct Block {
    number: String,
}

#[derive(Serialize)]
struct GraphqlQuery<'a> {
    query: &'a str,
}
fn main() {
    // let deployment_id = String::from("QmZHxmkc1iQoopStWvAEqjAvzbRmt6mrKRHHNRmmjyXbwr"); Tenderize
    let deployment_id = String::from("QmREeWJXhL4bNuu8Z1ZuxQkLdHQRcY1qKxQLQaEC98xnsR");

    let indexers = get_subgraph_indexers(&deployment_id);

    let mut min_block_behind: i64 = 100000000000000;

    match indexers {
        Ok(indexers) => {
            for i in &indexers {
                match get_indexer_status(&i.indexer.url, &deployment_id) {
                    Ok(block_behind) => {
                        println!("Blocks Behind: {}", block_behind);
                        min_block_behind = min(min_block_behind, block_behind);
                    }
                    Err(err) => {
                        println!("Failed ❌: {}", err);
                    }
                }
            }
        }
        Err(err) => {
            println!("Err: {}", err);
        }
    }
    println!("Min Block Behind: {}", min_block_behind);
}

#[tokio::main]
async fn get_subgraph_indexers(deployment_id: &String) -> Result<Vec<Indexer>, reqwest::Error> {
    const URL: &str =
        "https://api.thegraph.com/subgraphs/id/QmSWxvd8SaQK6qZKJ7xtfxCCGoRzGnoi2WNzmJYYJW9BXY";
    let client = reqwest::Client::new();

    let query = format!(
        r#"
    {{
        allocations(where:{{and: [ {{ status: Active }}, {{ subgraphDeployment_: {{ ipfsHash:"{}" }} }} ] }} ){{
            indexer {{
                url
            }}
        }}
    }}
"#,
        deployment_id
    );

    let req_body: GraphqlQuery = GraphqlQuery { query: &query };

    let response = client.post(URL).json(&req_body).send().await?;
    let response_json: Response<Allocations> = response.json().await?;

    Ok(response_json.data.allocations)
}

#[tokio::main]
async fn get_indexer_status(url: &String, deployment_id: &String) -> Result<i64, reqwest::Error> {
    let mut req_url = url.to_owned();
    if url.ends_with('/') {
        req_url += "status";
    } else {
        req_url += "/status";
    }
    println!("{}", req_url);
    let client = reqwest::Client::new();

    let query = format!(
        r#"
            {{
                indexingStatuses (subgraphs: ["{}"]) {{
                    chains {{
                            latestBlock {{ number }}
                            chainHeadBlock {{ number }}
                        }}
                    }}
            }}
        "#,
        deployment_id
    );

    let req_body = GraphqlQuery { query: &query };

    let response = client.post(req_url).json(&req_body).send().await?;
    let response_json: Response<IndexingStatus> = response.json().await?;

    let chain_head_block: i64 = response_json.data.indexingStatuses[0].chains[0]
        .chainHeadBlock
        .number
        .parse()
        .expect("Failed to parse chain head block");

    let latest_block: i64 = response_json.data.indexingStatuses[0].chains[0]
        .latestBlock
        .number
        .parse()
        .expect("Failed to parse latest block=");

    let blocks_behind = chain_head_block - latest_block;

    Ok(blocks_behind)
}
