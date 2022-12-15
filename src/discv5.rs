use crate::prelude::*;
use crate::types::discv5::{Enr, NodeId, NodeInfo};

#[cfg(any(feature = "client", feature = "server"))]
#[cfg_attr(feature = "client", rpc(client, namespace = "discv5"))]
#[cfg_attr(feature = "server", rpc(server, namespace = "discv5"))]
pub trait Discv5Api {
    /// Returns ENR and Node ID information of the local discv5 node.
    #[method(name = "nodeInfo")]
    async fn node_info(&self) -> RpcResult<NodeInfo>;

    /// Update the socket address of the local node record.
    #[method(name = "updateNodeInfo")]
    async fn update_node_info(
        &self,
        socket_addr: String,
        is_tcp: Option<bool>,
    ) -> RpcResult<NodeInfo>;

    /// Returns meta information about discv5 routing table.
    #[method(name = "routingTableInfo")]
    async fn routing_table_info(&self) -> RpcResult<NodeInfo>;

    /// Write an Ethereum Node Record to the routing table.
    #[method(name = "addEnr")]
    async fn add_enr(&self, enr: Enr) -> RpcResult<bool>;

    /// Fetch the latest ENR associated with the given node ID.
    #[method(name = "getEnr")]
    async fn get_enr(&self, node_id: NodeId) -> RpcResult<Enr>;

    /// Delete Node ID from the routing table.
    #[method(name = "deleteEnr")]
    async fn delete_enr(&self, node_id: NodeId) -> RpcResult<bool>;

    /// Fetch the ENR representation associated with the given Node ID and optional sequence number.
    #[method(name = "lookupEnr")]
    async fn lookup_enr(&self, node_id: NodeId, enr_seq: Option<u32>) -> RpcResult<Enr>;
}
