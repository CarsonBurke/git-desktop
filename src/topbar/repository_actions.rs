use iced::{
    alignment, widget::{button, column, container, row, scrollable, text, text_input}, Alignment, Command, Element, Length
};
use iced_aw::BootstrapIcon;

use crate::{
    constants::{font_sizes, spacings},
    general_widgets::{icons::bootstrap_icon, seperators::seperator_background_2},
    styles,
};

#[derive(Debug, Clone)]
pub enum RepositoryActionsMessage {
    FilterRepositories(String),
}

pub struct RepositoryActions {
    repositories_filter: String,
}

impl RepositoryActions {
    pub fn new() -> Self {
        Self {
            repositories_filter: String::new(),
        }
    }

    pub fn update(&mut self, message: RepositoryActionsMessage) -> Command<RepositoryActionsMessage> {
        match message {
            RepositoryActionsMessage::FilterRepositories(repositories_filter) => {
                self.repositories_filter = repositories_filter;
                
                Command::none()
            }
        }
    }

    pub fn view(&self) -> Element<RepositoryActionsMessage> {
        let content = column![
            row![
                text_input("Search", self.repositories_filter.as_str())
                    .width(Length::Fill)
                    .on_input(RepositoryActionsMessage::FilterRepositories),
                button(bootstrap_icon(BootstrapIcon::Plus)).style(iced::theme::Button::Custom(
                    Box::new(styles::button::Background4 {
                        display_as_pressed: false,
                    })
                )),
            ]
            .spacing(spacings::SMALL)
            .align_items(Alignment::Center),
            column![
                row![
                    bootstrap_icon(BootstrapIcon::Clock),
                    text("Recent").size(font_sizes::H2)
                ]
                .spacing(spacings::SMALL)
                .align_items(Alignment::Center),
                column![
                    container(
                        row![bootstrap_icon(BootstrapIcon::Book), "Repository"]
                            .spacing(spacings::SMALL)
                    )
                    .style(styles::container::test_background_4())
                    .padding(spacings::SMALL)
                    .width(Length::Fill),
                    seperator_background_2(),
                    container(
                        row![bootstrap_icon(BootstrapIcon::Book), "Repository"]
                            .spacing(spacings::SMALL)
                    )
                    .style(styles::container::test_background_4())
                    .padding(spacings::SMALL)
                    .width(Length::Fill),
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
            .spacing(spacings::SMALL),
            column![
                row![
                    bootstrap_icon(BootstrapIcon::Person),
                    text("Carson Burke").size(font_sizes::H2)
                ]
                .spacing(spacings::SMALL)
                .align_items(Alignment::Center),
                column![
                    container(
                        row![bootstrap_icon(BootstrapIcon::Book), "Repository"]
                            .spacing(spacings::SMALL)
                    )
                    .style(styles::container::test_background_4())
                    .padding(spacings::SMALL)
                    .width(Length::Fill),
                    seperator_background_2(),
                    container(
                        row![bootstrap_icon(BootstrapIcon::Book), "Repository"]
                            .spacing(spacings::SMALL)
                    )
                    .style(styles::container::test_background_4())
                    .padding(spacings::SMALL)
                    .width(Length::Fill),
                    seperator_background_2(),
                    container(
                        row![bootstrap_icon(BootstrapIcon::Book), "Repository"]
                            .spacing(spacings::SMALL)
                    )
                    .style(styles::container::test_background_4())
                    .padding(spacings::SMALL)
                    .width(Length::Fill),
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

        let container =
            scrollable(container(content).style(styles::container::test_background_3()));
        container.into()
    }
}
