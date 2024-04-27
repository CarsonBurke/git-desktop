use iced::{
    widget::{column, container, row, text},
    Element, Length,
};
use iced_aw::BootstrapIcon;

use crate::{
    constants::spacings,
    general_widgets::icons::bootstrap_icon,
    styles::{
        self,
        container::{rounded_corners, test_background_2, test_background_3},
    },
};

#[derive(Debug)]
pub enum VersionGraphMessage {}

pub struct VersionGraph {}

impl VersionGraph {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self) -> Element<VersionGraphMessage> {
        let top = container(
            row![bootstrap_icon(BootstrapIcon::Code), text("Branch name")]
                .padding(spacings::SMALL)
                .spacing(spacings::SMALL),
        )
        .style(styles::container::test_background_3())
        .width(Length::Fill);

        let main = container(row!["content"])
            .style(test_background_2())
            .padding(spacings::SMALL)
            .width(Length::Fill);

        let content = column![top, main,];
        let container = container(content).style(rounded_corners());
        container.into()
    }
}
