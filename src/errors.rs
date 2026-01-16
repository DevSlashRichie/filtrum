use thiserror::Error;

#[derive(Debug, Error)]
pub enum FilterParseError {
    #[error("invalid filter structure")]
    FilterStructure,
    #[error("invalid filter value")]
    Value,
    #[error("unknown filter")]
    UnknownFilter,
}
