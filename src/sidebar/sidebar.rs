use iced::{
    widget::{column, container, horizontal_space, row},
    Command, Element, Length,
};
use iced_aw::BootstrapIcon;

use crate::{
    constants::spacings, data::repositories::Repositories, general_widgets::icons::bootstrap_icon, styles::container::sidebar
};

use super::{
    changes::{Changes, ChangesMessage},
    create_commit::{self, CreateCommit, CreateCommitMessage},
    history_and_changes::HistoryAndChanges,
};

#[derive(Debug, Clone)]
pub enum SidebarMessage {
    CreateCommitMessage(CreateCommitMessage),
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
    history_and_changes: Option<HistoryAndChanges>,
    create_commit: Option<CreateCommit>,
}

pub struct Sidebar {
    // display_states: SidebarDisplayStates,
    children: SidebarChildren,
    test_child: Changes,
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
                history_and_changes: Some(HistoryAndChanges::new()),
                create_commit: Some(CreateCommit::new()),
            },
            test_child: Changes::new(),
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
        }
    }

    pub fn view(
        &self,
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
            }
        ]
        .spacing(spacings::MEDIUM);
        let container = container(content)
            .padding(spacings::SMALL)
            .height(Length::Fill)
            .width(Length::Shrink)
            .max_width(200)
            .style(sidebar());

        container.into()
    }
}
