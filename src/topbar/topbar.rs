use iced::{
    theme,
    widget::{column, container, horizontal_space, row, text},
    Alignment, Element,
};
use iced_aw::BootstrapIcon;

use crate::{
    constants::{custom_theme, font_sizes, spacings},
    general_widgets::icons::bootstrap_icon,
    styles,
};

#[derive(Debug)]
pub enum TopbarMessage {}

pub struct Topbar {}

impl Topbar {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self) -> Element<TopbarMessage> {
        let left = row![
            // Repository name
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
            // Branch name
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
            // Action
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
            // Shortform
            row![
                bootstrap_icon(BootstrapIcon::Pencil),
                bootstrap_icon(BootstrapIcon::Folder),
                bootstrap_icon(BootstrapIcon::Globe),
            ]
            .spacing(spacings::MEDIUM)
        ]
        .spacing(spacings::LARGE)
        .padding(spacings::SMALL).align_items(Alignment::Center);
        let right = row![
            bootstrap_icon(BootstrapIcon::Dash),
            bootstrap_icon(BootstrapIcon::X)
        ]
        .spacing(spacings::SMALL)
        .padding(spacings::SMALL);

        let content = row![left, horizontal_space(), right];
        let container = container(content).style(styles::container::topbar());
        container.into()
    }
}
