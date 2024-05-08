use iced::{
    alignment,
    border::Radius,
    theme,
    widget::{
        self, button, checkbox, column, container, horizontal_space, image, row, text, toggler,
        Column,
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
    styles::{
        self,
        container::{divider_background_2, test_background_3},
    },
    utils::get_icon_for_file_extension,
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
                    dropdown_arrow(self.expanded).size(font_sizes::H3)
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

                        let mut index = 0;

                        for changed_file in changed_files {
                            match index {
                                0 => {}
                                _ => {
                                    children.push(seperator_background_2().into());
                                }
                            };

                            children.push(
                                button(
                                    row![
                                        match get_icon_for_file_extension(
                                            changed_file.extension.clone()
                                        ) {
                                            Ok(icon) => {
                                                container(
                                                    image(image::Handle::from_memory(icon))
                                                        .width(18)
                                                        .height(18),
                                                )
                                            }
                                            Err(_) => {
                                                container(
                                                    bootstrap_icon(BootstrapIcon::Code)
                                                        .size(font_sizes::H2),
                                                )
                                            }
                                        },
                                        row![
                                            text(changed_file.relative_path.clone())
                                                .size(font_sizes::H2)
                                                .style(theme::Text::Color(custom_theme::GREY_TEXT)),
                                            text(changed_file.name.clone()).size(font_sizes::H2)
                                        ]
                                    ]
                                    .spacing(spacings::SMALL)
                                    .align_items(Alignment::Center),
                                )
                                .style(iced::theme::Button::Custom(Box::new(
                                    styles::button::Background3 {
                                        display_as_pressed: false,
                                    },
                                )))
                                .width(Length::Fill)
                                .on_press(ChangesMessage::ToggleFile)
                                .into(),
                            );

                            index += 1;
                        }

                        children
                    })
                    .spacing(spacings::XSMALL)
                    .padding(spacings::SMALL)
                }
                false => column![],
            }
        ])
        .style(test_background_3());
        let container = container(content);
        container.into()
    }
}
