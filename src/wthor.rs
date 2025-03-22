/// Parses bytes into one of three representations.
///
/// If a solitaire file is detected only the header will be returned.
pub fn parse<W: WthorFile>(bytes: &[u8]) -> Result<W::Output, Error> {
    W::parse(bytes)
}

pub trait WthorFile {
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
}
