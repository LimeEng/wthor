use crate::header::Header;
use encoding::{all::ISO_8859_1, DecoderTrap, Encoding};
use std::str::Utf8Error;

pub(crate) fn parse(
    record_size: usize,
    header: &Header,
    bytes: &[u8],
) -> Result<Vec<String>, NameFileError> {
    if header.n1 != 0 {
        return Err(NameFileError::InvalidN1Value);
    }
    if header.n2 == 0 {
        return Err(NameFileError::InvalidN2Value);
    }
    if header.p1 != 0 {
        return Err(NameFileError::InvalidP1Value);
    }
    if header.p2 != 0 {
        return Err(NameFileError::InvalidP2Value);
    }
    if header.n2 as usize * record_size != bytes.len() {
        return Err(NameFileError::SizeMismatch);
    }

    let mut names: Vec<String> = Vec::with_capacity(header.n2 as usize);
    let step = record_size;
    for i in 0..header.n2 {
        let start = (i as usize) * step;
        if bytes[start + step - 1] != 0 {
            return Err(NameFileError::RecordNotNullTerminated);
        }
        let record = &bytes[start..start + step];
        let decoded = ISO_8859_1.decode(record, DecoderTrap::Strict);
        let name = decoded.map_err(|_| NameFileError::EncodingNotIso8859_1)?;

        // Remove all nulls
        let name = name.trim_matches(char::from(0)).to_string();
        names.push(name);
    }

    Ok(names)
}

#[derive(Debug)]
pub enum NameFileError {
    IoError(std::io::Error),
    InvalidN1Value,
    InvalidN2Value,
    InvalidP1Value,
    InvalidP2Value,
    SizeMismatch,
    RecordNotNullTerminated,
    InvalidRecord(Utf8Error),
    EncodingNotIso8859_1,
}

impl From<std::io::Error> for NameFileError {
    fn from(error: std::io::Error) -> Self {
        NameFileError::IoError(error)
    }
}

impl From<Utf8Error> for NameFileError {
    fn from(error: Utf8Error) -> Self {
        NameFileError::InvalidRecord(error)
    }
}
