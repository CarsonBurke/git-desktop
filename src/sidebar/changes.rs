use iced::{
    alignment,
    border::Radius,
    theme,
    widget::{
        self, button, checkbox, column, container, horizontal_space, row, text, toggler, Column,
    },
    Alignment, Background, Border, Command, Element, Length,
};
use iced_aw::BootstrapIcon;

use crate::{
    constants::{
        custom_theme::{self, BACKGROUND_3, BACKGROUND_4},
        font_sizes, spacings,
    },
    data::repositories::ChangedFile,
    general_widgets::{
        icons::{bootstrap_icon, dropdown_arrow},
        seperators::{seperator_background_1, seperator_background_2},
        split_table_single::split_table_single,
    },
    styles::{self, container::divider_background_2},
};

use super::sidebar::DisplayState;

#[derive(Debug, Clone)]
pub enum ChangesMessage {
    ToggleExpanded,
    ToggleFile,
}

pub struct Changes {
    expanded: bool,
}

impl Changes {
    pub fn new() -> Self {
        Self { expanded: true }
    }

    pub fn update(&mut self, message: ChangesMessage) -> Command<ChangesMessage> {
        match message {
            ChangesMessage::ToggleExpanded => {
                self.expanded = !self.expanded;

                Command::none()
            }
            ChangesMessage::ToggleFile => Command::none(),
        }
    }

    pub fn view(&self, changed_files: Vec<ChangedFile>) -> Element<ChangesMessage> {
        let content = container(column![
            button(
                row![
                    container(text("6").size(font_sizes::H3))
                        .padding(spacings::XSMALL)
                        .style(styles::container::background3_rounded()),
                    text("Changes").size(font_sizes::H2),
                    horizontal_space(),
                    dropdown_arrow(self.expanded)
                ]
                .spacing(spacings::SMALL)
                .align_items(Alignment::Center)
            )
            .style(iced::theme::Button::Custom(Box::new(
                styles::button::Background3 {
                    display_as_pressed: self.expanded,
                }
            )))
            .on_press(ChangesMessage::ToggleExpanded)
            .padding(spacings::SMALL),
            match self.expanded {
                true => {
                    Column::with_children({
                        let mut children = Vec::new();

                        for changed_file in changed_files {
                            children.push(
                                column![button(
                                    row![
                                        bootstrap_icon(BootstrapIcon::Code).size(font_sizes::H2),
                                        row![
                                            text(changed_file.relative_path)
                                                .size(font_sizes::H2)
                                                .style(theme::Text::Color(custom_theme::GREY_TEXT)),
                                            text(changed_file.name).size(font_sizes::H2)
                                        ]
                                    ]
                                    .spacing(spacings::SMALL)
                                    .align_items(Alignment::Center)
                                )
                                .style(iced::theme::Button::Custom(Box::new(
                                    styles::button::Background4 {
                                        display_as_pressed: false,
                                    }
                                )))
                                .width(Length::Fill)
                                .on_press(ChangesMessage::ToggleFile),]
                                .spacing(spacings::SMALL)
                                .padding(spacings::SMALL)
                                .into(),
                            )
                        }

                        children
                    })
                }
                // column![
                //     button(
                //         row![
                //             bootstrap_icon(BootstrapIcon::Code).size(font_sizes::H2),
                //             text("File name").size(font_sizes::H2)
                //         ]
                //         .spacing(spacings::SMALL)
                //         .padding(spacings::SMALL)
                //         .align_items(Alignment::Center)
                //     )
                //     .style(iced::theme::Button::Custom(Box::new(
                //         styles::button::Background4 {
                //             display_as_pressed: false,
                //         }
                //     )))
                //     .width(Length::Fill)
                //     .on_press(ChangesMessage::ToggleFile),
                // ]
                // .spacing(spacings::SMALL)
                // .padding(spacings::SMALL),
                false => column![],
            }
        ])
        .style(changes_parent());
        let container = container(content);
        container.into()
    }
}

pub fn changes_parent() -> widget::container::Appearance {
    widget::container::Appearance {
        background: Some(Background::from(BACKGROUND_3)),
        border: Border {
            radius: 8.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn changes_header() -> widget::container::Appearance {
    widget::container::Appearance {
        background: Some(Background::from(BACKGROUND_4)),
        ..Default::default()
    }
}
