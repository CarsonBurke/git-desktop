use app::App;
use constants::ICON;
use iced::{self, advanced::graphics::image::image_rs::ImageFormat, Application, Settings};

mod pages;
mod sidebar;
mod app;
mod constants;
mod theme;
mod general_widgets;
mod styles;
mod topbar;

pub fn main() -> iced::Result{

    let icon = iced::window::icon::from_file_data(ICON, Some(ImageFormat::Png)).unwrap();

    let settings = Settings {
        window: iced::window::Settings {
            icon: Some(icon),
            transparent: true,
            // decorations: false,
            ..Default::default()
        },
        antialiasing: true,
        ..Default::default()
    };

    // env::set_var("RUST_BACKTRACE", "1");
    App::run(settings)
}