use iced::{
    theme,
    widget::{
        button, column, container, horizontal_space, row, scrollable, text, text_input, Column, Row,
    },
    Alignment, Command, Element, Length,
};
use iced_aw::BootstrapIcon;

use crate::{
    constants::{custom_theme, font_sizes, spacings},
    data::repositories::Branches,
    general_widgets::{icons::bootstrap_icon, seperators::seperator_background_2},
    styles,
};

#[derive(Debug, Clone)]
pub enum BranchActionsMessage {
    FilterBranches(String),
    NewBranch,
}

pub struct BranchActions {
    branches_filter: String,
}

impl BranchActions {
    pub fn new() -> Self {
        Self {
            branches_filter: "".to_string(),
        }
    }

    pub fn update(&mut self, message: BranchActionsMessage) -> Command<BranchActionsMessage> {
        match message {
            BranchActionsMessage::FilterBranches(branches_filter) => {
                self.branches_filter = branches_filter;

                Command::none()
            }
            BranchActionsMessage::NewBranch => {
                Command::none()
            }
        }
    }

    pub fn view(
        &self,
        branch_name: &Option<String>,
        branches: &Branches,
    ) -> Element<BranchActionsMessage> {
        let content = column![
            column![
                row![
                    text_input("Search", self.branches_filter.as_str())
                        .width(Length::Fill)
                        .on_input(BranchActionsMessage::FilterBranches).size(font_sizes::H3),
                    button(bootstrap_icon(BootstrapIcon::Plus)).style(iced::theme::Button::Custom(
                        Box::new(styles::button::Background4 {
                            display_as_pressed: false,
                        })
                    )).on_press(BranchActionsMessage::NewBranch),
                ]
                .spacing(spacings::SMALL)
                .align_items(Alignment::Center),
                column![
                    text("Primary branch")
                        .size(font_sizes::H3)
                        .style(theme::Text::Color(custom_theme::GREY_TEXT)),
                    text(
                        branch_name
                            .clone()
                            .unwrap_or("no default branch".to_string())
                    )
                    .size(font_sizes::H2)
                ],
            ]
            .spacing(spacings::SMALL),
            column![
                text("All Branches").size(font_sizes::H2),
                Column::with_children({
                    let mut children = Vec::new();
                    let mut i = 0;

                    for (branch_name, branch) in branches {
                        children.push(
                            column![
                                {
                                    match i == 0 {
                                        true => container(row![]),
                                        false => seperator_background_2(),
                                    }
                                },
                                container(row![
                                    text(branch_name).size(font_sizes::H2),
                                    horizontal_space(),
                                    text("yesterday")
                                        .size(font_sizes::H4)
                                        .style(theme::Text::Color(custom_theme::GREY_TEXT))
                                ].align_items(Alignment::Center))
                                .style(styles::container::test_background_4())
                                .width(Length::Fill)
                                .padding(spacings::SMALL)
                            ]
                            .into(),
                        );

                        i += 1;
                    }

                    children
                })
            ]
            .spacing(spacings::SMALL),
        ]
        .spacing(spacings::MEDIUM)
        .padding(spacings::SMALL);

        let container =
            scrollable(container(content).style(styles::container::test_background_3()));
        container.into()
    }
}
