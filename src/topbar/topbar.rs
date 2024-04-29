use iced::{
    theme,
    widget::{button, column, container, horizontal_space, row, text},
    Alignment, Element,
};
use iced_aw::BootstrapIcon;

use crate::{
    constants::{custom_theme, font_sizes, spacings},
    general_widgets::icons::bootstrap_icon,
    styles,
};

pub enum TopbarGitAction {
    Fetch,
    Pull,
    Push,
}

#[derive(Debug, Clone)]
pub enum TopbarMessage {
    RepositoryActions,
    BranchActions,
    Fetch,
    Pull,
    Push,
    OpenEditor,
    OpenFolder,
    OpenBrowser,
    MinimizeWindow,
    CloseWindow,
}

pub struct Topbar {}

impl Topbar {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, git_action: TopbarGitAction) -> Element<TopbarMessage> {
        let left = row![
            // Repository name
            button(
                row![
                    row![
                        bootstrap_icon(BootstrapIcon::Book),
                        column![
                            text("Current repository")
                                .size(font_sizes::H3)
                                .style(theme::Text::Color(custom_theme::GREY_TEXT)),
                            text("Repository Name").size(font_sizes::H2)
                        ]
                    ]
                    .spacing(spacings::SMALL)
                    .align_items(Alignment::Center),
                    bootstrap_icon(BootstrapIcon::ChevronDown),
                ]
                .spacing(spacings::MEDIUM)
                .align_items(Alignment::Center),
            )
            .on_press(TopbarMessage::RepositoryActions)
            .style(iced::theme::Button::Custom(Box::new(
                styles::button::Background1 {
                    display_as_pressed: false,
                }
            ))),
            // Branch name
            button(
                row![
                    row![
                        bootstrap_icon(BootstrapIcon::Alt),
                        column![
                            text("Current branch")
                                .size(font_sizes::H3)
                                .style(theme::Text::Color(custom_theme::GREY_TEXT)),
                            text("Branch Name").size(font_sizes::H2)
                        ]
                    ]
                    .spacing(spacings::SMALL)
                    .align_items(Alignment::Center),
                    bootstrap_icon(BootstrapIcon::ChevronDown),
                ]
                .spacing(spacings::MEDIUM)
                .align_items(Alignment::Center),
            )
            .on_press(TopbarMessage::BranchActions)
            .style(iced::theme::Button::Custom(Box::new(
                styles::button::Background1 {
                    display_as_pressed: false,
                }
            ))),
            // Action
            match git_action {
                TopbarGitAction::Fetch => {
                    button(
                        row![
                            bootstrap_icon(BootstrapIcon::CloudCheck),
                            column![
                                text("Fetch from origin").size(font_sizes::H2),
                                text("fetched 14 minutes")
                                    .size(font_sizes::H3)
                                    .style(theme::Text::Color(custom_theme::GREY_TEXT)),
                            ]
                        ]
                        .spacing(spacings::SMALL)
                        .align_items(Alignment::Center),
                    )
                    .on_press(TopbarMessage::Fetch)
                    .style(iced::theme::Button::Custom(Box::new(
                        styles::button::Background1 {
                            display_as_pressed: false,
                        },
                    )))
                }
                _ => {
                    button(
                        row![
                            bootstrap_icon(BootstrapIcon::CloudCheck),
                            column![
                                text("Action name").size(font_sizes::H2),
                                text("Action information")
                                    .size(font_sizes::H3)
                                    .style(theme::Text::Color(custom_theme::GREY_TEXT)),
                            ]
                        ]
                        .spacing(spacings::SMALL)
                        .align_items(Alignment::Center),
                    )
                    .on_press(TopbarMessage::Push)
                    .style(iced::theme::Button::Custom(Box::new(
                        styles::button::Background1 {
                            display_as_pressed: false,
                        },
                    )))
                }
            },
            // Shortform
            row![
                button(bootstrap_icon(BootstrapIcon::Pencil))
                    .on_press(TopbarMessage::OpenEditor)
                    .style(iced::theme::Button::Custom(Box::new(
                        styles::button::Background1 {
                            display_as_pressed: false,
                        }
                    ))),
                button(bootstrap_icon(BootstrapIcon::Folder))
                    .on_press(TopbarMessage::OpenFolder)
                    .style(iced::theme::Button::Custom(Box::new(
                        styles::button::Background1 {
                            display_as_pressed: false,
                        }
                    ))),
                button(bootstrap_icon(BootstrapIcon::Globe))
                    .on_press(TopbarMessage::OpenBrowser)
                    .style(iced::theme::Button::Custom(Box::new(
                        styles::button::Background1 {
                            display_as_pressed: false,
                        }
                    ))),
            ]
            .spacing(spacings::SMALL)
        ]
        .spacing(spacings::MEDIUM)
        .padding(spacings::SMALL)
        .align_items(Alignment::Center);
        let right = row![
            button(bootstrap_icon(BootstrapIcon::Dash))
                .on_press(TopbarMessage::MinimizeWindow)
                .style(iced::theme::Button::Custom(Box::new(
                    styles::button::Background3Circle {
                        display_as_pressed: false,
                    }
                ))),
            button(bootstrap_icon(BootstrapIcon::X))
                .on_press(TopbarMessage::CloseWindow)
                .style(iced::theme::Button::Custom(Box::new(
                    styles::button::Background3Circle {
                        display_as_pressed: false,
                    }
                ))),
        ]
        .spacing(spacings::SMALL)
        .padding(spacings::SMALL);

        let content = row![left, horizontal_space(), right];
        let container = container(content).style(styles::container::topbar());
        container.into()
    }
}
