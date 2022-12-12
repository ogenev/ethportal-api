mod web3;

pub use web3::*;

mod prelude {
    pub use jsonrpsee::core::RpcResult;
    pub use jsonrpsee::proc_macros::rpc;
}
