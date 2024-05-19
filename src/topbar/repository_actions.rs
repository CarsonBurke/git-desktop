use iced::{
    alignment, theme,
    widget::{
        button, column, container, horizontal_space, row, scrollable, text, text_input, Column,
    },
    Alignment, Command, Element, Length,
};
use iced_aw::BootstrapIcon;

use crate::{
    constants::{custom_theme, font_sizes, spacings},
    data::repositories::{Repositories, Repository},
    general_widgets::{
        icons::{bootstrap_icon, dropdown_arrow},
        seperators::{separator, seperator_background_1, seperator_background_2},
    },
    styles::{self, container::test_background_4},
};

#[derive(Debug, Clone)]
pub enum RepositoryActionsMessage {
    FilterRepositories(String),
    NewRepository,
    SelectRepository(String),
    ExpandRecent,
    ExpandUser,
}

pub struct RepositoryActions {
    repositories_filter: String,
    recent_expanded: bool,
    user_expanded: bool,
}

impl RepositoryActions {
    pub fn new() -> Self {
        Self {
            repositories_filter: String::new(),
            recent_expanded: true,
            user_expanded: false,
        }
    }

    pub fn update(
        &mut self,
        message: RepositoryActionsMessage,
    ) -> Command<RepositoryActionsMessage> {
        match message {
            RepositoryActionsMessage::FilterRepositories(repositories_filter) => {
                self.repositories_filter = repositories_filter;

                Command::none()
            }
            RepositoryActionsMessage::NewRepository => Command::none(),
            RepositoryActionsMessage::SelectRepository(repository_name) => Command::none(),
            RepositoryActionsMessage::ExpandRecent => {
                self.recent_expanded = !self.recent_expanded;

                Command::none()
            }
            RepositoryActionsMessage::ExpandUser => {
                self.user_expanded = !self.user_expanded;

                Command::none()
            }
        }
    }

    pub fn view(
        &self,
        current_repository_name: String,
        repositories: &Repositories,
    ) -> Element<RepositoryActionsMessage> {
        
        let filtered_repositories: Vec<(&String, &Repository)> = repositories
            .iter()
            .filter(|(repository_name, _)| {
                repository_name.contains(self.repositories_filter.as_str())
            })
            .collect();

        let content = column![
            row![
                text_input("Search", self.repositories_filter.as_str())
                    .width(Length::Fill)
                    .on_input(RepositoryActionsMessage::FilterRepositories)
                    .size(font_sizes::H3),
                button(bootstrap_icon(BootstrapIcon::Plus).size(font_sizes::H3))
                    .style(iced::theme::Button::Custom(Box::new(
                        styles::button::Background4 {
                            display_as_pressed: false,
                        }
                    )))
                    .on_press(RepositoryActionsMessage::NewRepository),
            ]
            .spacing(spacings::SMALL)
            .align_items(Alignment::Center),
            column![
                button(
                    row![
                        bootstrap_icon(BootstrapIcon::Clock).size(font_sizes::H2),
                        text("Recent").size(font_sizes::H2),
                        horizontal_space(),
                        dropdown_arrow(self.recent_expanded).size(font_sizes::H3),
                    ]
                    .spacing(spacings::SMALL)
                    .align_items(Alignment::Center),
                )
                .style(iced::theme::Button::Custom(Box::new(
                    styles::button::Background4 {
                        display_as_pressed: self.recent_expanded,
                    },
                )))
                .padding(spacings::SMALL)
                .on_press(RepositoryActionsMessage::ExpandRecent),
                match self.recent_expanded {
                    true => {
                        container(Column::with_children({
                            let mut children = Vec::new();
                            let mut i = 0;

                            for (repository_name, repository) in &filtered_repositories {
                                children.push(match i == 0 {
                                    true => container(row![]).into(),
                                    false => {
                                        separator(styles::container::test_background_3()).into()
                                    }
                                });

                                children.push(
                                    button(
                                        row![
                                            (|| {
                                                if repository_name == &&current_repository_name {
                                                    return bootstrap_icon(
                                                        BootstrapIcon::ArrowRight,
                                                    )
                                                    .size(font_sizes::H3);
                                                };

                                                text("")
                                            })(),
                                            text(repository_name).size(font_sizes::H3),
                                            horizontal_space(),
                                            text("•")
                                                .size(font_sizes::H3)
                                                .style(theme::Text::Color(custom_theme::GREY_TEXT))
                                        ]
                                        .spacing(spacings::SMALL)
                                        .align_items(Alignment::Center),
                                    )
                                    .style(iced::theme::Button::Custom(Box::new(
                                        styles::button::Background4 {
                                            display_as_pressed: false,
                                        },
                                    )))
                                    .width(Length::Fill)
                                    .on_press(RepositoryActionsMessage::SelectRepository(
                                        repository_name.to_string(),
                                    ))
                                    .into(),
                                );

                                i += 1;
                            }

                            children
                        }))
                        .padding(spacings::XSMALL)
                        .style(styles::container::test_background_4())
                    }
                    false => {
                        container(column![])
                    }
                }
            ],
            column![
                button(
                    row![
                        bootstrap_icon(BootstrapIcon::Person).size(font_sizes::H2),
                        text("Carson Burke").size(font_sizes::H2),
                        horizontal_space(),
                        dropdown_arrow(self.user_expanded).size(font_sizes::H3),
                    ]
                    .spacing(spacings::SMALL)
                    .align_items(Alignment::Center),
                )
                .style(iced::theme::Button::Custom(Box::new(
                    styles::button::Background4 {
                        display_as_pressed: self.user_expanded,
                    },
                )))
                .padding(spacings::SMALL)
                .on_press(RepositoryActionsMessage::ExpandUser),
                match self.user_expanded {
                    true => {
                        container(Column::with_children({
                            let mut children = Vec::new();
                            let mut i = 0;

                            for (repository_name, repository) in &filtered_repositories {
                                children.push(match i == 0 {
                                    true => container(row![]).into(),
                                    false => {
                                        separator(styles::container::test_background_3()).into()
                                    }
                                });

                                children.push(
                                    button(
                                        row![
                                            (|| {
                                                if repository_name == &&current_repository_name {
                                                    return bootstrap_icon(
                                                        BootstrapIcon::ArrowRight,
                                                    )
                                                    .size(font_sizes::H3);
                                                };

                                                text("")
                                            })(),
                                            text(repository_name).size(font_sizes::H3),
                                            horizontal_space(),
                                            text("•")
                                                .size(font_sizes::H3)
                                                .style(theme::Text::Color(custom_theme::GREY_TEXT))
                                        ]
                                        .spacing(spacings::SMALL)
                                        .align_items(Alignment::Center),
                                    )
                                    .style(iced::theme::Button::Custom(Box::new(
                                        styles::button::Background4 {
                                            display_as_pressed: false,
                                        },
                                    )))
                                    .width(Length::Fill)
                                    .on_press(RepositoryActionsMessage::SelectRepository(
                                        repository_name.to_string(),
                                    ))
                                    .into(),
                                );

                                i += 1;
                            }

                            children
                        }))
                        .padding(spacings::XSMALL)
                        .style(styles::container::test_background_4())
                    }
                    false => {
                        container(column![])
                    }
                }
            ],
        ]
        .spacing(spacings::MEDIUM)
        .padding(spacings::SMALL);

        let container =
            scrollable(container(content).style(styles::container::background3_rounded()));
        container.into()
    }
}
