mod models;
mod message;
mod backend;
mod ui;
mod utils;

use iced::{window, Size};
use ui::app::LibraApp;

pub fn main() -> iced::Result {
    let mut window_settings = window::Settings {
        size: Size::new(1280.0, 720.0),
        ..Default::default()
    };

    #[cfg(target_os = "macos")]
    {
        window_settings.platform_specific = window::settings::PlatformSpecific {
            titlebar_transparent: true,
            title_hidden: true,
            fullsize_content_view: true,
            ..Default::default()
        };
    }

    iced::application(LibraApp::title, LibraApp::update, LibraApp::view)
        .subscription(LibraApp::subscription)
        .theme(LibraApp::theme)
        .window(window_settings)
        .run_with(LibraApp::new)
}
