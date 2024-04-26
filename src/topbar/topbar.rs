use iced::{
    widget::{container, horizontal_space, row},
    Element,
};
use iced_aw::BootstrapIcon;

use crate::{constants::spacings, general_widgets::icons::bootstrap_icon, styles};

#[derive(Debug)]
pub enum TopbarMessage {}

pub struct Topbar {}

impl Topbar {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self) -> Element<TopbarMessage> {
        let left = row![
            bootstrap_icon(BootstrapIcon::Book),
        ].spacing(spacings::SMALL).padding(spacings::SMALL);
        let right = row![
            bootstrap_icon(BootstrapIcon::Dash),
            bootstrap_icon(BootstrapIcon::X)
        ]
        .spacing(spacings::SMALL).padding(spacings::SMALL);

        let content = row![left, horizontal_space(), right];
        let container = container(content).style(styles::container::topbar());
        container.into()
    }
}
