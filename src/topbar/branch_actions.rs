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
    data::repositories::{Branch, Branches},
    general_widgets::{
        icons::{bootstrap_icon, dropdown_arrow},
        seperators::{separator, seperator_background_1, seperator_background_2},
    },
    styles,
};

#[derive(Debug, Clone)]
pub enum BranchActionsMessage {
    FilterBranches(String),
    NewBranch,
    SelectBranch(String),
    ExpandAllBranches,
}

#[derive(Debug, Clone)]
pub struct BranchActions {
    branches_filter: String,
    all_branches_expanded: bool,
}

impl BranchActions {
    pub fn new() -> Self {
        Self {
            branches_filter: "".to_string(),
            all_branches_expanded: true,
        }
    }

    pub fn update(&mut self, message: BranchActionsMessage) -> Command<BranchActionsMessage> {
        match message {
            BranchActionsMessage::FilterBranches(branches_filter) => {
                self.branches_filter = branches_filter.to_lowercase();

                Command::none()
            }
            BranchActionsMessage::NewBranch => Command::none(),
            BranchActionsMessage::SelectBranch(branch_name) => Command::none(),
            BranchActionsMessage::ExpandAllBranches => {
                self.all_branches_expanded = !self.all_branches_expanded;

                Command::none()
            }
        }
    }

    pub fn view(
        &self,
        current_branch_name: &Option<String>,
        branches: &Branches,
    ) -> Element<BranchActionsMessage> {

        let filtered_branches: Vec<(&String, &Branch)> = branches
        .iter()
        .filter(|(repository_name, _)| {
            repository_name.to_lowercase().contains(self.branches_filter.as_str())
        })
        .collect();

        let content = column![
            column![
                row![
                    text_input("Search", self.branches_filter.as_str())
                        .width(Length::Fill)
                        .on_input(BranchActionsMessage::FilterBranches)
                        .size(font_sizes::H3),
                    button(bootstrap_icon(BootstrapIcon::Plus).size(font_sizes::H3))
                        .style(iced::theme::Button::Custom(Box::new(
                            styles::button::Background4 {
                                display_as_pressed: false,
                            }
                        )))
                        .on_press(BranchActionsMessage::NewBranch),
                ]
                .spacing(spacings::SMALL)
                .align_items(Alignment::Center),
                column![
                    text("Primary branch")
                        .size(font_sizes::H3)
                        .style(theme::Text::Color(custom_theme::GREY_TEXT)),
                    text(
                        current_branch_name
                            .clone()
                            .unwrap_or("no default branch".to_string())
                    )
                    .size(font_sizes::H2)
                ],
            ]
            .spacing(spacings::SMALL),
            column![
                button(
                    row![
                        bootstrap_icon(BootstrapIcon::Alt).size(font_sizes::H2),
                        text("All Branches").size(font_sizes::H2),
                        horizontal_space(),
                        dropdown_arrow(self.all_branches_expanded).size(font_sizes::H3),
                    ]
                    .spacing(spacings::SMALL)
                    .align_items(Alignment::Center),
                )
                .style(iced::theme::Button::Custom(Box::new(
                    styles::button::Background4 {
                        display_as_pressed: self.all_branches_expanded,
                    },
                )))
                .padding(spacings::SMALL)
                .on_press(BranchActionsMessage::ExpandAllBranches),
                match self.all_branches_expanded {
                    true => {
                        container(Column::with_children({
                            let mut children = Vec::new();
                            let mut i = 0;

                            for (branch_name, branch) in &filtered_branches {
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
                                                let Some(current_branch_name) = current_branch_name
                                                else {
                                                    return text("");
                                                };

                                                if branch_name == &current_branch_name {
                                                    return bootstrap_icon(
                                                        BootstrapIcon::ArrowRight,
                                                    )
                                                    .size(font_sizes::H3);
                                                };

                                                text("")
                                            })(),
                                            text(branch_name).size(font_sizes::H3),
                                            horizontal_space(),
                                            text("yesterday")
                                                .size(font_sizes::H4)
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
                                    .on_press(BranchActionsMessage::SelectBranch(
                                        branch_name.to_string(),
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
                        container(row![])
                    }
                }
            ]
        ]
        .spacing(spacings::MEDIUM)
        .padding(spacings::SMALL);

        let container =
            scrollable(container(content).style(styles::container::background3_rounded()));
        container.into()
    }
}
