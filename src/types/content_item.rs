use crate::types::accumulator::EpochAccumulator;
use crate::types::block_body::BlockBody;
use crate::types::block_header::{BlockHeader, BlockHeaderWithProof};
use crate::types::receipts::BlockReceipts;
use serde::{Deserialize, Serialize};

/// Portal History content items. Support BlockHeaderWithProof and depreciated BlockHeader
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HistoryContentItem {
    BlockHeaderWithProof(Box<BlockHeaderWithProof>),
    BlockHeader(Box<BlockHeader>),
    BlockBody(BlockBody),
    Receipts(BlockReceipts),
    EpochAccumulator(EpochAccumulator),
}
