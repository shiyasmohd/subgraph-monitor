use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

const DEPLOYMENT_IDS: [&str; 75] = [
    "QmcKTiRPK6wthULv25XLwy1rCgrCDj9FV1YHdiYYKBzPFN",
    "QmRTofBwy8LMrAmZJiXEkMgKCtWz9DzxxHJcSYqFLYj9W8",
    "QmVQ9jq4bBgfGKzKBL1AjYFGTqLEAYqTPUbs9YienYewpf",
    "QmVeQLQJzy1T21UtKncpLs8nNjRczKz5sUT94h515QhwuJ",
    "QmZkT9mzDf5YcbSti51BZGet7sKnNcYAjBBE9xQ3LWHqKC",
    "Qmcdd4SGVwG9VY4itrXBnBPWBVsQvsxfzaib9bVny9C8jT",
    "QmQXFxg4dCxMqcobGQAtsNe4ufnG1KmjSVgtEdtxvYo4Lf",
    "QmT25bz8HV5173wbZ21vcMR76R34BW7Bix4fES9iibBKKW",
    "QmdMmVsnaVRxQyunaXrG1oFrpcUFp8iV3TcpRa3hVYaPUD",
    "QmcTYTs3DCbxF97gyo1tq54Vkd9YUDcSjPNUB1rRKBotrM",
    "QmfLnYXftuf2PNPoC3mMusey27KyJi4UUuY741Nz3vZQ3q",
    "QmZC9vYPNAdew2U4qAYXCjBGwfrpwvxKRm3RE2njwzwgxE",
    "Qmf7PkcBd251aKyxCcMPRBWKc76tZ8k1xmVMSZ5zKqoSbh",
    "QmR6VP1qoF8nxhtMaGEg9VMmkaTDFqyeM8nJWkakP6nHes",
    "QmSDPnHzyW8yfnuhB423ssVY5r4bQrr5C1rXT8qMroNgmv",
    "QmWZpwizHCefGY3vBUJhDhaVHPUgYE8oAAyBy1T5HATbMb",
    "QmQLtMYbTvxDHqc5oFH75N2b4DL3Czdv2jSCo6bk4UwKyf",
    "QmYyjZXjfUaqe359vzGgxe2ju1qJPiQM68pG2ipfNL8F7F",
    "QmVbYM6wz7XnH32QzQLNBwyzx7r1HNsFP6jfUi22NRvvJu",
    "QmXU45n3iB7AKQWLwAMs2MaGkkMjqF7sF8QW32NxQKFLx9",
    "QmUwneRXVUqTsPEZqScQytg257Rij3nSApe4pdDAtuG4s4",
    "QmQfYe5Ygg9A3mAiuBZYj5a64bDKLF4gF6sezfhgxKvb9y",
    "QmZG6wYBro1aU5Sg2V3J3n6omQcJCAZsmnMoJNp68Em4s2",
    "QmRLTuSKgS8GbioD9vfAmn9dUUm5uVPptUdFMS5LG4yzAA",
    "QmajmWiw8tyhgJcGk69QiL2gEXqniM3PNPAaLjPh6dYfpf",
    "QmbBFHznvVVj5FHA32nHCJ3EEbGBFV7gbnEJ7MvcfJSWRg",
    "QmamfQF1cQMWgYVWM7vYq1yTL5fTrkr2ZYCAYR9rMsEWV5",
    "QmakqW3KCpEXJGTSuyfXHGg6C8ppbrPdSqVXWj3Guor9zA",
    "QmWX1tYofCvtw675s6ccE8ovWDkuikRBkkLk4PQpT79jRD",
    "QmbQQsmYqNoVdWG9fpffvcqeg6zvwDjUxE3T65AB1Lmmha",
    "QmQV6TaRFgBdk4ykzpf6mfdWVB2GLVjkidJZ2YWctR2w2C",
    "QmV4FT95YqNdbivbPmvjXRavdKu3FGkz41mjuqtvUvpRpc",
    "QmRcpZhksXHJjkAf7Kk9ngGQteWUNAEJ1VH3jttivdN2ft",
    "QmZdYzMxLNHM5u8YjhoMdQUfNHDC4KxwBVAB3XAk9WSaui",
    "QmV5qTnwjz65z6TH63DYngLr1gj52KQ6Lr6m8aeNj6yxxb",
    "QmevuQAmgiou1UX7Z2BQf6RKdX1WunQjH8VtawhkzSLj4V",
    "Qma2FYbCR7iBQmGaGN4oLMB5TtnBcN9bgTttYyt3vD6xXX",
    "QmW6L18iCzUeBsNJXyjQC2abZWeafPYMXr4ZVGYQvDGUsa",
    "QmdChdTYcr6YgfKHw6FCUtJsfZBEQkvsoAkiRAs1GcGWMZ",
    "QmSuZ3L62PyVC4RjYFikoYZhyDMGMkPNUArVMaj9G73Dxw",
    "QmT3VpmrGD7bbPGT6aKqaK4yg3u73ryhgp5kEM2BVTEWFj",
    "QmbcWZNLLCSmuQ474rYr4jCymN22jguT9hZtp4rZzA3T2w",
    "QmRqdvBLBWVw2JtnDj2JAtZnVkTt4DvGBuiFtphvVYAe9U",
    "QmPpECMB4vnHe2SpkLQCFp7dumWfXUekhZBbfVAEM64BjS",
    "QmdnkNhFRuDcshaD6RXTf4zSqHntUtnHGr2krUtbzfF9F7",
    "QmcHkq534QMFaGWsPqkeKeuGLwZwVKK7KRqZE3cKxaEojH",
    "QmecR8pDHLNaRGQtKxA1ZvSSyBh23haXPdYZppPzALQmzt",
    "QmPbbjK9vtY69kSxMJRVLVz1dRzUBNwYbWTRYMiFzp9Tzm",
    "QmfFsgEPtw6mCkaaLxk2e52E7G2KryTzn6R1eTYoL3Mj1c",
    "QmZ2R9ABG9ienaZdGyPLcDWDNDkG187RhXmh6fFuEtUaaS",
    "QmQLU8dEJcxYj7rVGbGWRYHrhKifbQKTtPRfKhHw1aszWh",
    "QmRGvRxvfNgVNU9QcTDj6XpASfA3HaYCGJiKqRmthGdjm2",
    "QmbT6ZsxJEZKUrbzLrAWV5EnQfSKZeYtZ265mQUaB552vK",
    "QmcRAdiUuYzZDauyM2q8FDucZaDVLgKPGsMyKn1ds4DXfm",
    "QmWisRwB5h2fWMUdGbxQEdqMvVWAiZB1BxsCdoqcsn2Cij",
    "QmNZ9ePvxGRDHAEhb7cLsb3AvtCCPJ3qAwh1CDvDn39RMa",
    "QmWH5ChjmF4Yp5Yhiaxczh5QwbG6HFSEi8bRwbKaUrJA6C",
    "QmUCx4QRvmDiP1fJaE8RQ72YkkXP9c62rckyhf9EPh87aj",
    "QmY9S4TVSMX6bcjVnvRhntMUS6ppdv23ktNhLQkWm5PaVn",
    "QmaZo8AP9HQPXrnVUP1outjfsb3xJW1PF9X6UHjzid1aYH",
    "QmStbnz4sErrK1jzaU5iKAutA1Q2w47EsgAoFbdCiequtZ",
    "QmTbcaVrmr3VBKNhNjrqkhLo2WtPBA7Ana9eqj4PD216K1",
    "QmRjFd4SPZDpMdjkPQqgN9v4trWXpQoqikFrmqpBUMiTJY",
    "QmXsPN4TD4PUhT1ZWd5d1mdQPePFNMdJwUr6guSh1z9ZzA",
    "QmRcpZhksXHJjkAf7Kk9ngGQteWUNAEJ1VH3jttivdN2ft",
    "QmcrH1y6zx6wzTBL9cKVdA81fHkPzytcx5Gy1iVJLP1Vfw",
    "QmS9uxga2rzpWyHKFATGYas6ntdHLpX7w45EKAatBTZJ25",
    "QmajRCaeDeftPiNKndmqCRMknKLNC6sFXBREWX4HHyfzv8",
    "QmauqkgAJTjwfKDAvtY4siuPZF2oczg6jqWC9wMqw21yQH",
    "QmQ2h69a3vnE6N3TN7Ys9K1vpjYiJSi8fexnj1pWpRc6uY",
    "QmQHLPkCwBjc4GUEDEHC1eZAXPzVHrs3uf3PEM7bFodhwA",
    "QmNwmGQw2vcK991B7MJCJgsZAS8Mt2fQZ41qrPMJFVKaAU",
    "QmQdLeKXfkgjE35QmBNTeEac4Fa4SvqYZ9wWJF43Nwv8KH",
    "QmVNMMgTVAJ5f3GSAASS5eYsGrcudmsCrXJyF9j5v9d5eC",
    "QmdCukF6WX46K2Lgy4QJR8AyKQByc2NCRAUAyRxm9uA4uZ",
];
const BLOCK_BEHIND: i64 = 0;

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
