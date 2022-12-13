use crate::prelude::*;
use crate::types::discv5::NodeInfo;

#[cfg(any(feature = "client", feature = "server"))]
#[cfg_attr(feature = "client", rpc(client, namespace = "web3"))]
#[cfg_attr(feature = "server", rpc(server, namespace = "web3"))]
pub trait Discv5Api {
    #[method(name = "nodeInfo")]
    async fn node_info(&self) -> RpcResult<NodeInfo>;
}
