use std::str::FromStr;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(super) struct RawParticipant {
    pub(super) name: String,
    pub(super) family: usize,
    pub(super) last_year_targets: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub(super) n_gifts: usize,
    pub(super) participants: Vec<RawParticipant>,
}

impl FromStr for Config {
    type Err = json5::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        json5::from_str(s)
    }
}