use iced::{
    theme,
    widget::{button, column, container, horizontal_space, row, scrollable, text},
    Alignment, Command, Element,
};
use iced_aw::{BootstrapIcon, DropDown};

use crate::{
    constants::{custom_theme, font_sizes, spacings},
    data::repositories::{self, Repositories, Repository},
    general_widgets::icons::{bootstrap_icon, dropdown_arrow},
    pages::repository,
    styles,
};

use super::{
    branch_actions::{BranchActions, BranchActionsMessage},
    repository_actions::{self, RepositoryActions, RepositoryActionsMessage},
};

pub enum TopbarGitAction {
    Fetch,
    Pull,
    Push,
}

#[derive(Debug, Clone)]
pub enum TopbarMessage {
    ToggleRepositoryActions,
    ToggleBranchActions,
    Fetch,
    Pull,
    Push,
    OpenEditor,
    OpenFolder,
    OpenBrowser,
    RepositoryActionsMessage(RepositoryActionsMessage),
    BranchActionsMessage(BranchActionsMessage),
}

pub struct Topbar {
    repository_actions: Option<RepositoryActions>,
    branch_actions: Option<BranchActions>,
}

impl Topbar {
    pub fn new() -> Self {
        Self {
            repository_actions: None,
            branch_actions: None,
        }
    }

    fn create_repository_actions() -> Option<RepositoryActions> {
        Some(RepositoryActions::new())
    }

    fn create_branch_actions() -> Option<BranchActions> {
        Some(BranchActions::new())
    }

    pub fn update(
        &mut self,
        message: TopbarMessage,
        git_action: TopbarGitAction,
        repository_name: &Option<String>,
        repositories: &Repositories,
    ) -> Command<TopbarMessage> {
        match message {
            TopbarMessage::ToggleRepositoryActions => {
                if self.repository_actions.is_some() {
                    self.repository_actions = None;

                    Command::none()
                } else {
                    self.repository_actions = Self::create_repository_actions();
                    self.branch_actions = None;

                    Command::none()
                }
            }
            TopbarMessage::ToggleBranchActions => {
                if self.branch_actions.is_some() {
                    self.branch_actions = None;

                    Command::none()
                } else {
                    self.branch_actions = Self::create_branch_actions();
                    self.repository_actions = None;

                    Command::none()
                }
            }
            TopbarMessage::RepositoryActionsMessage(repository_actions_message) => {
                let Some(repository_actions) = &mut self.repository_actions else {
                    return Command::none();
                };

                let _ = repository_actions.update(repository_actions_message);

                Command::none()
            }
            TopbarMessage::BranchActionsMessage(branch_actions_message) => {
                let Some(branch_actions) = &mut self.branch_actions else {
                    return Command::none();
                };

                let _ = branch_actions.update(branch_actions_message);

                Command::none()
            }
            TopbarMessage::OpenBrowser => {
                let Some(repository_name) = repository_name else {
                    println!("[Error] Expected repository name");
                    return Command::none();
                };

                let Some(repository) = repositories.get(repository_name) else {
                    println!("[Error] Expected repository");
                    return Command::none();
                };

                let _ = webbrowser::open(repository.url.as_str());

                Command::none()
            }
            TopbarMessage::OpenFolder => {
                let Some(repository_name) = repository_name else {
                    println!("[Error] Expected repository name");
                    return Command::none();
                };

                let Some(repository) = repositories.get(repository_name) else {
                    println!("[Error] Expected repository");
                    return Command::none();
                };

                let _ = opener::open(repository.file_path.as_str());

                Command::none()
            }
            _ => Command::none(),
        }
    }

    fn no_repository_view(&self) -> Element<TopbarMessage> {
        let content = row![text("no repository")];

        let container = container(content).style(styles::container::topbar());
        container.into()
    }

    pub fn view(
        &self,
        git_action: TopbarGitAction,
        repository_name: &Option<String>,
        repositories: &Repositories,
    ) -> Element<TopbarMessage> {
        let Some(repository_name) = repository_name else {
            return self.no_repository_view();
        };

        let Some(repository) = repositories.get(repository_name) else {
            return self.no_repository_view();
        };

        let left = row![
            // Repository name
            DropDown::new(
                button(
                    row![
                        row![
                            bootstrap_icon(BootstrapIcon::Book),
                            column![
                                text("Current repository")
                                    .size(font_sizes::H3)
                                    .style(theme::Text::Color(custom_theme::GREY_TEXT)),
                                text(repository_name).size(font_sizes::H2)
                            ]
                        ]
                        .spacing(spacings::SMALL)
                        .align_items(Alignment::Center),
                        dropdown_arrow(self.repository_actions.is_some()),
                    ]
                    .spacing(spacings::MEDIUM)
                    .align_items(Alignment::Center),
                )
                .on_press(TopbarMessage::ToggleRepositoryActions)
                .style(iced::theme::Button::Custom(Box::new(
                    styles::button::Background1 {
                        display_as_pressed: self.repository_actions.is_some(),
                    }
                ))),
                match &self.repository_actions {
                    Some(repository_actions) => {
                        repository_actions
                            .view()
                            .map(TopbarMessage::RepositoryActionsMessage)
                    }
                    None => {
                        text("none").into()
                    }
                },
                self.repository_actions.is_some()
            ),
            // Branch name
            DropDown::new(
                button(
                    row![
                        row![
                            bootstrap_icon(BootstrapIcon::Alt),
                            column![
                                text("Current branch")
                                    .size(font_sizes::H3)
                                    .style(theme::Text::Color(custom_theme::GREY_TEXT)),
                                text(
                                    repository
                                        .current_branch
                                        .clone()
                                        .unwrap_or("no default branch".to_string())
                                )
                                .size(font_sizes::H2)
                            ]
                        ]
                        .spacing(spacings::SMALL)
                        .align_items(Alignment::Center),
                        dropdown_arrow(self.branch_actions.is_some()),
                    ]
                    .spacing(spacings::MEDIUM)
                    .align_items(Alignment::Center),
                )
                .on_press(TopbarMessage::ToggleBranchActions)
                .style(iced::theme::Button::Custom(Box::new(
                    styles::button::Background1 {
                        display_as_pressed: self.branch_actions.is_some(),
                    }
                ))),
                match &self.branch_actions {
                    Some(branch_actions) => {
                        branch_actions
                            .view(&repository.current_branch, &repository.branches)
                            .map(TopbarMessage::BranchActionsMessage)
                    }
                    None => {
                        text("none").into()
                    }
                },
                self.branch_actions.is_some()
            ),
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
        let right = row![].spacing(spacings::SMALL).padding(spacings::SMALL);

        let content = row![left, horizontal_space(), right];

        let container = container(content).style(styles::container::topbar());
        container.into()
    }
}
