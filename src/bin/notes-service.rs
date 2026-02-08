// SPDX-License-Identifier: MPL-2.0

use notes_basic::{
    app::{ServiceFlags, ServiceModel},
    i18n,
};

fn main() -> cosmic::iced::Result {
    tracing_subscriber::fmt::init();

    // Get the system's preferred languages.
    let requested_languages = i18n_embed::DesktopLanguageRequester::requested_languages();

    // Enable localizations to be applied.
    i18n::init(&requested_languages);

    // Settings for configuring the application window and iced runtime.
    let settings = cosmic::app::Settings::default()
        .size_limits(
            cosmic::iced::Limits::NONE
                .min_width(300.0)
                .min_height(200.0),
        )
        .debug(true)
        .exit_on_close(false)
        .no_main_window(true);

    // Starts the application's event loop with `()` as the application's flags.
    //cosmic::app::run::<ServiceModel>(settings, ServiceFlags)
    cosmic::app::run_single_instance::<ServiceModel>(settings, ServiceFlags)
}
