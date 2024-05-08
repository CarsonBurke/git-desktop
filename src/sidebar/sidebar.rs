use iced::{
    widget::{column, container, horizontal_space, row},
    Command, Element, Length,
};
use iced_aw::BootstrapIcon;

use crate::{
    constants::spacings, data::repositories::{ChangedFile, Repositories}, general_widgets::icons::bootstrap_icon, styles::container::sidebar
};

use super::{
    changes::{Changes, ChangesMessage},
    create_commit::{self, CreateCommit, CreateCommitMessage},
    history_and_changes::HistoryAndChanges,
};

#[derive(Debug, Clone)]
pub enum SidebarMessage {
    CreateCommitMessage(CreateCommitMessage),
    ChangesMessage(ChangesMessage),
}

pub enum DisplayState {
    Visible,
    Hidden,
}

// deal

// struct SidebarDisplayStates {
//     history_and_changes: Child,
// }

pub struct SidebarChildren {
    changes: Option<Changes>,
    create_commit: Option<CreateCommit>,
}

pub struct Sidebar {
    // display_states: SidebarDisplayStates,
    children: SidebarChildren,
}

impl Sidebar {
    pub fn new() -> Self {
        Self {
            // display_states: SidebarDisplayStates {
            //     history_and_changes: Child {
            //         display: DisplayState::Visible,
            //     },
            // },
            children: SidebarChildren {
                changes: Some(Changes::new()),
                create_commit: Some(CreateCommit::new()),
            },
        }
    }

    pub fn update(&mut self, message: SidebarMessage) -> Command<SidebarMessage> {
        match message {
            SidebarMessage::CreateCommitMessage(create_commit_message) => {
                let Some(create_commit) = &mut self.children.create_commit else {
                    return Command::none();
                };

                let _ = create_commit
                    .update(create_commit_message)
                    .map(SidebarMessage::CreateCommitMessage);

                Command::none()
            }
            SidebarMessage::ChangesMessage(changes_message) => {
                let Some(changes) = &mut self.children.changes else {
                    return Command::none()
                };

                let _ = changes.update(changes_message).map(SidebarMessage::ChangesMessage);

                Command::none()
            }
        }
    }

    pub fn view(
        &self,
        divider_position: Option<u16>,
        repository_name: &Option<String>,
        repositories: &Repositories,
    ) -> Element<SidebarMessage> {

        let Some(repository_name) = repository_name else {
            return container(row![]).into()
        };

        let Some(repository) = repositories.get(repository_name) else {
            return container(row![]).into()
        };

        let topbar = row![
            bootstrap_icon(BootstrapIcon::Lamp),
            horizontal_space(),
            bootstrap_icon(BootstrapIcon::Gear),
        ];

        let content = column![
            topbar,
            match &self.children.create_commit {
                Some(create_commit) => create_commit
                    .view(repository.current_branch.clone().unwrap_or("[Error] no branch".to_string()))
                    .map(SidebarMessage::CreateCommitMessage),
                None => column![].into(),
            },
            match &self.children.changes {
                Some(changes) => changes
                    .view({
                        let mut changed_files = Vec::new();

                        changed_files.push(ChangedFile {
                            name: "main".to_string(),
                            extension: "rs".to_string(),
                            relative_path: "src/".to_string(),
                        });
                        changed_files.push(ChangedFile {
                            name: "README".to_string(),
                            extension: "txt".to_string(),
                            relative_path: "/".to_string(),
                        });

                        changed_files
                    })
                    .map(SidebarMessage::ChangesMessage),
                None => column![].into(),
            },
        ]
        .spacing(spacings::MEDIUM);
        let container = container(content)
            .padding(spacings::SMALL)
            .height(Length::Fill)
            .width(Length::Shrink)
            .max_width(divider_position.unwrap_or(200))
            .style(sidebar());

        container.into()
    }
}
