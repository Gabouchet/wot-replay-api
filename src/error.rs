use std::env::VarError;
use wot_extractor::errors::ExtractError;
use wot_replay_parser::ReplayError;

#[derive(thiserror::Error, Debug)]
pub enum ReplayApiError {
    #[error("Failed to fetch replay")]
    ReplayFetchError(#[from] reqwest::Error),

    #[error("Unable to parse the replay file")]
    ReplayDecodeError(#[from] ReplayError),

    #[error("Unable to parse the replay file")]
    ReplayJsonDecodeError,

    #[error("Unable to decode scalars in the replay file")]
    ReplayConvertError(#[from] std::num::ParseIntError),

    #[error("Unable to fetch game information")]
    GameDataFetchError(#[from] ExtractError),

    #[error("Bad configuration")]
    BadConfigurationError(#[from] VarError),
}