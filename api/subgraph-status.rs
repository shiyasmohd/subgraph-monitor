use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

const DEPLOYMENT_IDS: [&str; 3] = [
    "QmcKTiRPK6wthULv25XLwy1rCgrCDj9FV1YHdiYYKBzPFN",
    "QmRTofBwy8LMrAmZJiXEkMgKCtWz9DzxxHJcSYqFLYj9W8",
    "QmVQ9jq4bBgfGKzKBL1AjYFGTqLEAYqTPUbs9YienYewpf",
];
const BLOCK_BEHIND: i64 = 500;

#[derive(Deserialize, Debug)]
struct ApiResponse<T> {
    data: T,
}

#[derive(Deserialize, Serialize, Debug)]
struct Indexer {
    url: String,
}
#[derive(Deserialize, Serialize, Debug)]
struct Subgraph {
    ipfsHash: String,
}
#[derive(Deserialize, Serialize, Debug)]
struct Allocation {
    indexer: Indexer,
    subgraphDeployment: Subgraph,
}
#[derive(Deserialize, Serialize, Debug)]
struct Allocations {
    allocations: Vec<Allocation>,
}

#[derive(Debug)]
struct SubgraphStatus {
    deployment_id: String,
    blocks_behind: i64,
}

#[derive(Serialize)]
struct GraphqlQuery<'a> {
    query: &'a str,
    variables: Variables<'a>,
}

#[derive(Serialize)]
struct Variables<'a> {
    subgraphs: &'a Vec<String>,
}

#[derive(Deserialize, Debug)]
struct IndexingStatuses {
    indexingStatuses: Vec<IndexingStatus>,
}

#[derive(Deserialize, Debug)]
struct IndexingStatus {
    subgraph: String,
    chains: Vec<Chain>,
}
#[derive(Deserialize, Debug)]
struct Chain {
    latestBlock: Block,
    chainHeadBlock: Block,
}

#[derive(Deserialize, Debug)]
struct Block {
    number: String,
}

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let indexers = get_indexers().await?;

    let mut subgraph_statuses: HashMap<String, i64> = HashMap::new();

    for indexer in indexers {
        let subgraphs = get_subgraph_status_by_indexer(indexer.0, indexer.1).await?;
        for subgraph in subgraphs {
            subgraph_statuses
                .entry(subgraph.deployment_id)
                .and_modify(|curr_block_behind| {
                    if subgraph.blocks_behind < *curr_block_behind {
                        *curr_block_behind = subgraph.blocks_behind
                    }
                })
                .or_insert(subgraph.blocks_behind);
        }
    }

    let mut entries_to_remove = Vec::new();
    for (deployment_id, blocks_behind) in &mut subgraph_statuses {
        if *blocks_behind < BLOCK_BEHIND + 1 {
            entries_to_remove.push(deployment_id.clone());
        }
    }

    for deployment_id in entries_to_remove {
        subgraph_statuses.remove(&deployment_id);
    }

    let mut message = format!("Subgraphs behind {} Blocks\n\n", BLOCK_BEHIND);
    for (key, value) in &subgraph_statuses {
        message.push_str(&format!("{} : {} Blocks behind\n", key, value));
    }

    println!("Message \n{}", message);

    send_bot_message(message).await?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
              "message": "subgraph_statuses"
            })
            .to_string()
            .into(),
        )?)
}

async fn get_indexers() -> Result<HashMap<String, Vec<String>>, reqwest::Error> {
    let mut indexer_map: HashMap<String, Vec<String>> = HashMap::new();

    let query_url = format!(
    "https://gateway-arbitrum.network.thegraph.com/api/{}/deployments/id/QmSWxvd8SaQK6qZKJ7xtfxCCGoRzGnoi2WNzmJYYJW9BXY",
    std::env::var("GRAPH_API_KEY").unwrap()
    );

    let query = format!(
        r#"
        query indexers($subgraphs: [String!]!){{
            allocations(where: {{ status:Active, subgraphDeployment_: {{ ipfsHash_in: $subgraphs }} }}){{
                indexer {{
                    url
                }}
                subgraphDeployment {{
                    ipfsHash
                }}
            }}	
        }}
        "#
    );

    let subgraphs = DEPLOYMENT_IDS
        .iter()
        .map(|&deployment_id| deployment_id.to_string())
        .collect();

    let req_body = GraphqlQuery {
        query: &query,
        variables: Variables {
            subgraphs: &subgraphs,
        },
    };

    let client = reqwest::Client::new();
    let response = client.post(query_url).json(&req_body).send().await?;

    let response_json: ApiResponse<Allocations> = response.json().await?;

    for allocation in &response_json.data.allocations {
        let indexer_url = &allocation.indexer.url;
        let ipfs_hash = &allocation.subgraphDeployment.ipfsHash;

        indexer_map
            .entry(indexer_url.clone())
            .or_insert_with(Vec::new)
            .push(ipfs_hash.clone());
    }

    println!("{:?}", indexer_map);

    Ok(indexer_map)
}

async fn get_subgraph_status_by_indexer(
    indexer_url: String,
    deployment_ids: Vec<String>,
) -> Result<Vec<SubgraphStatus>, reqwest::Error> {
    let query = format!(
        r#"
        query SubgraphStatus($subgraphs: [String!]!){{
            indexingStatuses (subgraphs: $subgraphs) {{
                subgraph
                chains {{
                    latestBlock {{ number }}
                    chainHeadBlock {{ number }}
                }}
            }}
        }}
        "#
    );

    let req_body = GraphqlQuery {
        query: &query,
        variables: Variables {
            subgraphs: &deployment_ids,
        },
    };

    let mut url = String::from("");
    if indexer_url.ends_with("/") {
        url = indexer_url + "status";
    } else {
        url = indexer_url + "/status";
    }

    println!("{}", url);

    let client = reqwest::Client::builder().use_rustls_tls().build()?;
    let response = client.post(url.clone()).json(&req_body).send().await;
    let mut subgraph_statuses: Vec<SubgraphStatus> = vec![];

    match response {
        Ok(response) => {
            let response_json: Result<ApiResponse<IndexingStatuses>, _> = response.json().await;

            match response_json {
                Ok(res_json) => {
                    for subgraph in res_json.data.indexingStatuses {
                        let chain_head: i64 = subgraph.chains[0]
                            .chainHeadBlock
                            .number
                            .parse()
                            .expect("Not a vaild Number");
                        let latest_block: i64 = subgraph.chains[0]
                            .latestBlock
                            .number
                            .parse()
                            .expect("Not a valid number");
                        println!("Chain Head: {}", chain_head);
                        println!("Latest Block: {}", latest_block);
                        subgraph_statuses.push(SubgraphStatus {
                            deployment_id: subgraph.subgraph,
                            blocks_behind: chain_head - latest_block,
                        });
                    }
                }
                Err(err) => println!("Failed to fetch status from {}", url),
            }
        }
        Err(err) => println!("Failed to fetch status from {}", url),
    }

    Ok(subgraph_statuses)
}

async fn send_bot_message(message: String) -> Result<(), reqwest::Error> {
    let chat_id = String::from(std::env::var("TG_CHAT_ID").unwrap());
    let tg_api_url = format!(
        "https://api.telegram.org/bot{}/sendMessage",
        std::env::var("TG_BOT_TOKEN").unwrap()
    );
    let params = [("chat_id", chat_id), ("text", message)];
    let client = reqwest::Client::new();
    client.get(tg_api_url).query(&params).send().await?;
    Ok(())
}
