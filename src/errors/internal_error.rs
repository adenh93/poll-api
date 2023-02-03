use super::error_chain_fmt;

#[derive(thiserror::Error)]
pub enum InternalError {
    #[error("{0}")]
    ParseIpError(&'static str),
}

impl std::fmt::Debug for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}
