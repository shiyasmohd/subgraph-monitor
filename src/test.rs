#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().use_rustls_tls().build()?;

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse()?);

    let data = r#"
{
    "query": "query SubgraphStatus($subgraphs: [String!]!){\n    indexingStatuses (subgraphs: $subgraphs) {\n        subgraph\n        chains {\n            latestBlock { number }\n            chainHeadBlock { number }\n        }\n    }\n}",
    "variables": {
    "subgraphs":[
    "QmSWxvd8SaQK6qZKJ7xtfxCCGoRzGnoi2WNzmJYYJW9BXY"
]
}
}
"#;
    let json: serde_json::Value = serde_json::from_str(&data)?;

    let request = client
        .request(
            reqwest::Method::POST,
            "https://service.thegraph.arbitrum.suntzu.pro/status",
        )
        .headers(headers)
        .json(&json);

    let response = request.send().await?;
    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}
