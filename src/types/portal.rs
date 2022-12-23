use crate::prelude::*;
use crate::types::content_key::HistoryContentKey;
use crate::types::discv5::Enr;

pub type DataRadius = ethereum_types::U256;
pub type Distance = ethereum_types::U256;

// TODO: Expand he following types to more complete structures
pub type ContentItem = String;
pub type BitList = String;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeInfo {
    enr: Enr,
    distance: Distance,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PongInfo {
    enr_seq: u32,
    data_radius: DataRadius,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContentInfo {
    #[serde(rename_all = "camelCase")]
    ConnectionId { connection_id: u16 },
    #[serde(rename_all = "camelCase")]
    Content { content: ContentItem },
    #[serde(rename_all = "camelCase")]
    Enrs { enrs: Vec<Enr> },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AcceptInfo {
    connection_id: u16,
    content_keys: BitList,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TraceContentInfo {
    content: ContentItem,
    route: Vec<NodeInfo>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginateLocalContentInfo {
    content_keys: Vec<HistoryContentKey>,
    total_entries: u64,
}
