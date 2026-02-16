// SPDX-License-Identifier: MPL-2.0

use crate::fl;
use cosmic::prelude::*;
use cosmic::{
    iced::{Alignment, widget::column},
    widget,
};
use std::str::FromStr;
use thiserror::Error;
pub use {
    applet::AppletModel,
    service::{ServiceFlags, ServiceModel},
    utils::to_f32,
};

mod about_window;
mod applet;
mod edit_style;
mod restore_view;
mod service;
mod settings_view;
mod sticky_window;
mod styles_view;
mod utils;

const APP_ID: &str = "com.github.aae.sticky_notes";

/// Messages emitted by the application and its widgets.
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Ping,
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
    OpenAbout,
}

#[derive(Debug, Error)]
pub enum NotesAppError {
    // Failed reading source file
    #[error("Failed parsing command: {0}")]
    ParseError(String),
}

const PING: &str = "PING";
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
const ABOUT: &str = "ABOUT";

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Command::Ping => PING,
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
                Command::OpenAbout => ABOUT,
            }
        )
    }
}

impl FromStr for Command {
    type Err = NotesAppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            PING => Ok(Self::Ping),
            QUIT => Ok(Self::Quit),
            LOAD => Ok(Self::LoadNotes),
            SAVE => Ok(Self::SaveNotes),
            IMPORT => Ok(Self::ImportNotes),
            EXPORT => Ok(Self::ExportNotes),
            SHOW => Ok(Self::ShowAllNotes),
            HIDE => Ok(Self::HideAllNotes),
            LOCK => Ok(Self::LockAll),
            RESTORE => Ok(Self::RestoreNotes),
            SETTINGS => Ok(Self::OpenSettings),
            ABOUT => Ok(Self::OpenAbout),
            _ => Err(NotesAppError::ParseError(s.to_string())),
        }
    }
}

fn build_main_popup_view<F, P, M>(
    core: &cosmic::Core,
    to_message: F,
    is_enabled: P,
) -> Element<'_, M>
where
    // such requirements to Message are defined in cosmic::Application:
    M: Clone + std::fmt::Debug + Send + 'static,
    // converter Command -> Message like Message::Signal(Command::Ping):
    F: Fn(Command) -> M,
    // tests if menu item should be enabled
    P: Fn(Command) -> bool,
{
    use cosmic::applet as cosmic_applet;

    let mut save_load = widget::column::with_capacity(2);
    if is_enabled(Command::LoadNotes) {
        save_load = save_load.push(
            cosmic_applet::menu_button(widget::text::body(fl!("load")))
                .on_press(to_message(Command::LoadNotes)),
        );
    }
    if is_enabled(Command::SaveNotes) {
        save_load = save_load.push(
            cosmic_applet::menu_button(widget::text::body(fl!("save")))
                .on_press(to_message(Command::SaveNotes)),
        );
    }

    let mut import_export = widget::column::with_capacity(2);
    if is_enabled(Command::ImportNotes) {
        import_export = import_export.push(
            cosmic_applet::menu_button(widget::text::body(fl!("import")))
                .on_press(to_message(Command::ImportNotes)),
        );
    }
    if is_enabled(Command::ExportNotes) {
        import_export = import_export.push(
            cosmic_applet::menu_button(widget::text::body(fl!("export")))
                .on_press(to_message(Command::ExportNotes)),
        );
    }

    let mut show_lock = widget::column::with_capacity(3);
    if is_enabled(Command::ShowAllNotes) {
        show_lock = show_lock.push(
            cosmic_applet::menu_button(widget::text::body(fl!("show-all")))
                .on_press(to_message(Command::ShowAllNotes)),
        );
    }
    if is_enabled(Command::HideAllNotes) {
        show_lock = show_lock.push(
            cosmic_applet::menu_button(widget::text::body(fl!("hide-all")))
                .on_press(to_message(Command::HideAllNotes)),
        );
    }
    if is_enabled(Command::LockAll) {
        show_lock = show_lock.push(
            cosmic_applet::menu_button(widget::text::body(fl!("lock-all")))
                .on_press(to_message(Command::LockAll)),
        );
    }

    let mut settings_restore = widget::column::with_capacity(4);
    if is_enabled(Command::RestoreNotes) {
        settings_restore = settings_restore.push(
            cosmic_applet::menu_button(widget::text::body(fl!("restore-notes")))
                .on_press(to_message(Command::RestoreNotes)),
        );
    }
    if is_enabled(Command::OpenSettings) {
        settings_restore = settings_restore.push(
            cosmic_applet::menu_button(widget::text::body(fl!("settings")))
                .on_press(to_message(Command::OpenSettings)),
        );
    }
    if is_enabled(Command::OpenAbout) {
        settings_restore = settings_restore.push(
            cosmic_applet::menu_button(widget::text::body(fl!("about")))
                .on_press(to_message(Command::OpenAbout)),
        );
    }
    if is_enabled(Command::Quit) {
        settings_restore = settings_restore.push(
            cosmic_applet::menu_button(widget::text::body(fl!("quit")))
                .on_press(to_message(Command::Quit)),
        );
    }

    let spacing = cosmic::theme::spacing();
    let content = column![
        save_load,
        cosmic_applet::padded_control(widget::divider::horizontal::default())
            .padding([spacing.space_xxs, spacing.space_s]),
        import_export,
        cosmic_applet::padded_control(widget::divider::horizontal::default())
            .padding([spacing.space_xxs, spacing.space_s]),
        show_lock,
        cosmic_applet::padded_control(widget::divider::horizontal::default())
            .padding([spacing.space_xxs, spacing.space_s]),
        settings_restore
    ]
    .align_x(Alignment::Start)
    .padding([8, 0]);

    core.applet
        .popup_container(content)
        .max_height(500.)
        .max_width(500.)
        .into()
}
