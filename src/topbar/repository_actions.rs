use iced::{
    alignment,
    widget::{column, container, row, text, text_input},
    Alignment, Element,
};
use iced_aw::BootstrapIcon;

use crate::{
    constants::spacings,
    general_widgets::{
        icons::bootstrap_icon,
        seperators::{seperator_background_1, seperator_background_2},
    },
    styles,
};

#[derive(Debug, Clone)]
pub enum RepositoryActionsMessage {}

pub struct RepositoryActions {}

impl RepositoryActions {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self) -> Element<RepositoryActionsMessage> {
        let content = column![
            container(
                row![
                    bootstrap_icon(BootstrapIcon::Search),
                    text_input("Search", "")
                ]
                .spacing(spacings::SMALL)
            )
            .align_y(alignment::Vertical::Center),
            row![bootstrap_icon(BootstrapIcon::Clock), text("Recent")].spacing(spacings::SMALL),
            column![
                container(
                    row![bootstrap_icon(BootstrapIcon::Book), "Repository"]
                        .spacing(spacings::SMALL)
                )
                .style(styles::container::test_background_2())
                .padding(spacings::SMALL),
                seperator_background_1(),
                container(
                    row![bootstrap_icon(BootstrapIcon::Book), "Repository"]
                        .spacing(spacings::SMALL)
                )
                .style(styles::container::test_background_2())
                .padding(spacings::SMALL),
                seperator_background_1(),
                container(
                    row![bootstrap_icon(BootstrapIcon::Book), "Repository"]
                        .spacing(spacings::SMALL)
                )
                .style(styles::container::test_background_2())
                .padding(spacings::SMALL),
            ],
            row![bootstrap_icon(BootstrapIcon::Person), text("Carson Burke")]
                .spacing(spacings::SMALL),
            column![
                container(
                    row![bootstrap_icon(BootstrapIcon::Book), "Repository"]
                        .spacing(spacings::SMALL)
                )
                .style(styles::container::test_background_2())
                .padding(spacings::SMALL),
                seperator_background_1(),
                container(
                    row![bootstrap_icon(BootstrapIcon::Book), "Repository"]
                        .spacing(spacings::SMALL)
                )
                .style(styles::container::test_background_2())
                .padding(spacings::SMALL),
                seperator_background_1(),
                container(
                    row![bootstrap_icon(BootstrapIcon::Book), "Repository"]
                        .spacing(spacings::SMALL)
                )
                .style(styles::container::test_background_2())
                .padding(spacings::SMALL),
                seperator_background_1(),
                container(
                    row![bootstrap_icon(BootstrapIcon::Book), "Repository"]
                        .spacing(spacings::SMALL)
                )
                .style(styles::container::test_background_2())
                .padding(spacings::SMALL),
            ],
        ]
        .spacing(spacings::SMALL)
        .padding(spacings::SMALL);
        let container = container(content);
        container.into()
    }
}
