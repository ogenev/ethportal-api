use crate::types::accumulator::EpochAccumulator;
use crate::types::block_body::BlockBody;
use crate::types::block_header::BlockHeader;
use crate::types::receipts::BlockReceipts;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HistoryContentItem {
    BlockHeader(Box<BlockHeader>),
    BlockBody(BlockBody),
    Receipts(BlockReceipts),
    EpochAccumulator(EpochAccumulator),
}
