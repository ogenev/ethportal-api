use crate::prelude::*;

type EnrBase64 = String;
pub type NodeId = [u8; 32];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeInfo {
    enr: EnrBase64,
    node_id: NodeId,
}
