use iced::{
    alignment,
    widget::{button, column, container, horizontal_space, row, text, text_editor, text_input},
    Alignment, Command, Element, Length,
};
use iced_aw::BootstrapIcon;

use crate::{
    constants::{font_sizes, spacings},
    general_widgets::icons::{bootstrap_icon, dropdown_arrow},
    styles,
};

#[derive(Debug, Clone)]

pub enum CreateCommitMessage {
    ToggleExpanded,
    CommitTitleChanged(String),
    CommitDescriptionChanged(String),
    Commit,
}

pub struct CreateCommit {
    expanded: bool,
    commit_title: String,
    commit_description: String,
}

impl CreateCommit {
    pub fn new() -> Self {
        Self {
            expanded: true,
            commit_title: String::new(),
            commit_description: String::new(),
        }
    }

    pub fn update(&mut self, message: CreateCommitMessage) -> Command<CreateCommitMessage> {
        match message {
            CreateCommitMessage::ToggleExpanded => {
                self.expanded = !self.expanded;

                Command::none()
            }
            CreateCommitMessage::CommitTitleChanged(title) => {
                self.commit_title = title;

                Command::none()
            }
            CreateCommitMessage::CommitDescriptionChanged(description) => {
                self.commit_description = description;

                Command::none()
            }
            CreateCommitMessage::Commit => {
                println!("Commit");

                Command::none()
            }
        }
    }

    pub fn view(&self, branch_name: String) -> Element<CreateCommitMessage> {
        let content = column![
            button(row![
                text("Commit").size(font_sizes::H2),
                horizontal_space(),
                dropdown_arrow(self.expanded)
            ])
            .style(iced::theme::Button::Custom(Box::new(
                styles::button::Background3 {
                    display_as_pressed: self.expanded,
                }
            )))
            .on_press(CreateCommitMessage::ToggleExpanded)
            .padding(spacings::SMALL),
            match self.expanded {
                true => {
                    column![
                        row![
                            container(text("C"))
                                .style(styles::container::test_background_4())
                                .padding(spacings::SMALL),
                            text_input("Title", self.commit_title.as_str())
                                .on_input(CreateCommitMessage::CommitTitleChanged)
                                .size(font_sizes::P)
                        ]
                        .align_items(Alignment::Center),
                        text_input("Description", self.commit_description.as_str())
                            .on_input(CreateCommitMessage::CommitDescriptionChanged)
                            .size(font_sizes::P),
                        button(
                            row![
                                horizontal_space(),
                                bootstrap_icon(BootstrapIcon::CloudArrowUp),
                                text(format!("Commit to {}", branch_name)).size(font_sizes::H3),
                                horizontal_space(),
                            ]
                            .spacing(spacings::SMALL)
                            .align_items(Alignment::Center)
                        )
                        .style(iced::theme::Button::Custom(Box::new(
                            styles::button::Primary {
                                display_as_pressed: self.expanded,
                            }
                        )))
                        .on_press(CreateCommitMessage::Commit)
                        .width(Length::Fill),
                    ]
                    .spacing(spacings::SMALL)
                    .padding(spacings::SMALL)
                }
                false => {
                    column![]
                }
            }
        ];
        let container = container(content).style(styles::container::test_background_3());
        container.into()
    }
}
