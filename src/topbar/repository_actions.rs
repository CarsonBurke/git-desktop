use iced::{
    alignment,
    widget::{column, container, row, scrollable, text, text_input},
    Alignment, Element, Length,
};
use iced_aw::BootstrapIcon;

use crate::{
    constants::spacings,
    general_widgets::{
        icons::bootstrap_icon,
        seperators::{seperator_background_2},
    },
    styles,
};

#[derive(Debug, Clone, Copy)]
pub enum RepositoryActionsMessage {}

pub struct RepositoryActions {}

impl RepositoryActions {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self) -> Element<RepositoryActionsMessage> {
        let content = column![
            row![
                bootstrap_icon(BootstrapIcon::Search),
                text_input("Search", "").width(Length::Fill)
            ]
            .spacing(spacings::SMALL)
            .align_items(Alignment::Center),
            column![
                row![bootstrap_icon(BootstrapIcon::Clock), text("Recent")].spacing(spacings::SMALL),
                column![
                    container(
                        row![bootstrap_icon(BootstrapIcon::Book), "Repository"]
                            .spacing(spacings::SMALL)
                    )
                    .style(styles::container::test_background_4())
                    .padding(spacings::SMALL).width(Length::Fill),
                    seperator_background_2(),
                    container(
                        row![bootstrap_icon(BootstrapIcon::Book), "Repository"]
                            .spacing(spacings::SMALL)
                    )
                    .style(styles::container::test_background_4())
                    .padding(spacings::SMALL).width(Length::Fill),
                    seperator_background_2(),
                    container(
                        row![bootstrap_icon(BootstrapIcon::Book), "Repository"]
                            .spacing(spacings::SMALL)
                    )
                    .style(styles::container::test_background_4())
                    .padding(spacings::SMALL).width(Length::Fill),
                ],
            ]
            .spacing(spacings::SMALL),
            column![
                row![bootstrap_icon(BootstrapIcon::Person), text("Carson Burke")]
                    .spacing(spacings::SMALL),
                column![
                    container(
                        row![bootstrap_icon(BootstrapIcon::Book), "Repository"]
                            .spacing(spacings::SMALL)
                    )
                    .style(styles::container::test_background_4())
                    .padding(spacings::SMALL).width(Length::Fill),
                    seperator_background_2(),
                    container(
                        row![bootstrap_icon(BootstrapIcon::Book), "Repository"]
                            .spacing(spacings::SMALL)
                    )
                    .style(styles::container::test_background_4())
                    .padding(spacings::SMALL).width(Length::Fill),
                    seperator_background_2(),
                    container(
                        row![bootstrap_icon(BootstrapIcon::Book), "Repository"]
                            .spacing(spacings::SMALL)
                    )
                    .style(styles::container::test_background_4())
                    .padding(spacings::SMALL).width(Length::Fill),
                    seperator_background_2(),
                    container(
                        row![bootstrap_icon(BootstrapIcon::Book), "Repository"]
                            .spacing(spacings::SMALL)
                    )
                    .style(styles::container::test_background_4())
                    .padding(spacings::SMALL)
                    .width(Length::Fill),
                ],
            ]
            .spacing(spacings::SMALL)
        ]
        .spacing(spacings::MEDIUM)
        .padding(spacings::SMALL);

        let container = scrollable(container(content).style(styles::container::test_background_3()));
        container.into()
    }
}
