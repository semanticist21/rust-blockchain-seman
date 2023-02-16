#[derive(Debug)]
pub enum BlockValidationError {
    MismatchedIndex,
    InvalidHash,
    AchronologicalTimestamp,
    MismatchedPreviousHash,
    InvalidGenesisBlock,
    InvalidInput,
    InsufficientInputVal,
    InvalidConinbaseTransaction,
}
