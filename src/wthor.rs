pub(crate) trait WthorFile {
    type Output;
    fn parse(bytes: &[u8]) -> Result<Self::Output, Error>;
}

#[derive(Clone, Debug)]
pub enum Error {
    // Header errors
    InvalidHeader,
    InvalidN1Value,
    InvalidN2Value,
    InvalidP2Value,
    InvalidP1Value,
    SizeMismatch,
    // GameArchive (Record) errors
    InvalidMove,
    InvalidSize,
    // Records errors
    RecordNotNullTerminated,
    EncodingNotIso8859_1,
    // Enrichment errors
    PlayerIndexOutOfBounds,
    TournamentIndexOutOfBounds,
}
