use bevy::prelude::*;
use thiserror::Error;

use crate::map_builder;

#[derive(Error, Debug)]
pub enum GameError {
    #[error("MapDraftError: {0}")]
    MapDraftError(#[from] map_builder::MapDraftError),
}

pub enum Severity {
    // Only log the error in as warning and continue with the game.
    Warning,

    // Only log the error as error and continue with the game.
    Error,

    // Log the error as error and exit the game.
    Critical,
}

impl GameError {
    pub fn severity(&self) -> Severity {
        match self {
            GameError::MapDraftError(_) => Severity::Critical,
        }
    }
}

pub fn error_handler(result: In<Result<(), GameError>>) {
    match result.as_ref() {
        Ok(_) => (),
        Err(e) => match e.severity() {
            Severity::Warning => bevy::log::warn!("Warning: {}", e),
            Severity::Error => bevy::log::error!("Error: {}", e),
            Severity::Critical => {
                bevy::log::error!("Critical: {}", e);
                panic!("Critical: {}", e);
            }
        },
    }
}
