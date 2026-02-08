// SPDX-License-Identifier: MPL-2.0

use std::str::FromStr;
use thiserror::Error;
pub use {
    applet::AppletModel,
    service::{ServiceFlags, ServiceModel},
    utils::to_f32,
};

mod applet;
mod edit_style;
mod restore_view;
mod service;
mod settings_view;
mod sticky_window;
mod styles_view;
mod utils;

/// Messages emitted by the application and its widgets.
#[derive(Debug, Clone)]
pub enum Command {
    Quit,
    LoadNotes,
    SaveNotes,
    ImportNotes,
    ExportNotes,
    ShowAllNotes,
    HideAllNotes,
    LockAll,
    RestoreNotes,
    OpenSettings,
}

#[derive(Debug, Error)]
pub enum NotesAppError {
    // Failed reading source file
    #[error("Failed parsing command: {0}")]
    ParseError(String),
}

const QUIT: &str = "QUIT";
const LOAD: &str = "LOAD";
const SAVE: &str = "SAVE";
const IMPORT: &str = "IMPORT";
const EXPORT: &str = "EXPORT";
const SHOW: &str = "SHOW";
const HIDE: &str = "HIDE";
const LOCK: &str = "LOCK";
const RESTORE: &str = "RESTORE";
const SETTINGS: &str = "SETTINGS";

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Command::Quit => QUIT,
                Command::LoadNotes => LOAD,
                Command::SaveNotes => SAVE,
                Command::ImportNotes => IMPORT,
                Command::ExportNotes => EXPORT,
                Command::ShowAllNotes => SHOW,
                Command::HideAllNotes => HIDE,
                Command::LockAll => LOCK,
                Command::RestoreNotes => RESTORE,
                Command::OpenSettings => SETTINGS,
            }
        )
    }
}

impl FromStr for Command {
    type Err = NotesAppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tmp = match s {
            QUIT => Self::Quit,
            LOAD => Self::LoadNotes,
            SAVE => Self::SaveNotes,
            IMPORT => Self::ImportNotes,
            EXPORT => Self::ExportNotes,
            SHOW => Self::ShowAllNotes,
            HIDE => Self::HideAllNotes,
            LOCK => Self::LockAll,
            RESTORE => Self::RestoreNotes,
            SETTINGS => Self::OpenSettings,
            _ => return Err(NotesAppError::ParseError(s.to_string())),
        };
        Ok(tmp)
    }
}
