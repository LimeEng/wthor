#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::convert::TryFrom;

use crate::{
    records::Records,
    wthor::{Error, WthorFile},
};

/// A collection of tournament names parsed from a `.trn` file.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub struct Tournaments {
    pub(crate) names: Vec<String>,
}

impl Tournaments {
    /// Parse tournament names from a `.trn` file.
    ///
    /// # Errors
    ///
    /// Returns an error if the bytes cannot be parsed as a valid WTHOR records file.
    pub(crate) fn parse(bytes: &[u8]) -> Result<Self, Error> {
        let records = Records::parse(bytes)?;
        Ok(Self {
            names: records.names,
        })
    }

    pub(crate) fn get(&self, index: u16) -> Option<&str> {
        let idx = index.checked_sub(1)? as usize;
        self.names.get(idx).map(String::as_str)
    }
}

impl TryFrom<&[u8]> for Tournaments {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Tournaments::parse(bytes)
    }
}
