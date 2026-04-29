use thiserror::Error;

#[derive(Debug, Error)]
pub enum EvmDecodeError {
    #[error("RPC error: {0}")]
    Rpc(String),
    #[error("Decode error: {0}")]
    Decode(String),
    #[error("Not found: {0}")]
    NotFound(String),
}
