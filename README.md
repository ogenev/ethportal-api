# ethportal-api
This crate contains definitions for various Portal Network JSON-RPC APIs using [jsonrpsee](https://github.com/paritytech/jsonrpsee) framework.

## Client usage example
Enable `client` feature of `ethportal-api` crate.

```rust,no_run
use ethportal_api::{Web3Api, HistoryNetworkApi, HistoryContentKey, HistoryContentItem};
use jsonrpsee::http_client::HttpClientBuilder;

#[tokio::main]
async fn main() {
    // Connect to a local node JSON-RPC
    let client = HttpClientBuilder::default().build("http://localhost:8545").unwrap();

    // Call web3_clientVersion endpoint
    let client_version = client.client_version().await.unwrap();
    println!("Current client version is {client_version}");
    
    let content_key = r#""0x04cb5cab7266694daa0d28cbf40496c08dd30bf732c41e0455e7ad389c10d79f4f"";
    // Deserialise to a portal history content key type from a hex string
    let content_key: HistoryContentKey = serde_json::from_str(&content_key).unwrap();
    
    // Call portal_historyLocalContent endpoint and deserialize to HistoryContentItem::AccumulatoProof type
    let content_item: HistoryContentItem = client.local_content(content_key).await.unwrap();
}
```

## License
The entire code within this repository is licensed under the [GNU General Public License v3.0](./LICENSE)