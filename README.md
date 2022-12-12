# ethportal-api
This crate contains definitions for various Portal Network JSON-RPC APIs using [jsonrpsee](https://github.com/paritytech/jsonrpsee) framework.

## Client usage example
Enable `client` feature of `ethportal-api` crate.

```rust,no_run
use ethportal_api::Web3Api;
use jsonrpsee::http_client::HttpClientBuilder;

#[tokio::main]
async fn main() {
    let client = HttpClientBuilder::default().build("http://localhost:8545").unwrap();

    let client_version = client.client_version().await.unwrap();
    println!("Current client version is {client_version}");
}
```

## License
The entire code within this repository is licensed under the [GNU General Public License v3.0](./LICENSE)