use iced::{
    widget::{column, container, horizontal_space, row}, Command, Element, Length
};
use iced_aw::BootstrapIcon;

use crate::{
    constants::spacings, general_widgets::icons::bootstrap_icon, styles::container::sidebar,
};

use super::{
    changes::{Changes, ChangesMessage},
    create_commit::{CreateCommit, CreateCommitMessage},
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
    history_and_changes: HistoryAndChanges,
    create_commit: CreateCommit,
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
                history_and_changes: HistoryAndChanges::new(),
                create_commit: CreateCommit::new(),
            },
            test_child: Changes::new(),
        }
    }

    pub fn update(&mut self, message: SidebarMessage) -> Command<SidebarMessage> {
        match message {
            SidebarMessage::CreateCommitMessage(create_commit_message) => {
                self.children.create_commit.update(create_commit_message).map(SidebarMessage::CreateCommitMessage)
            }
        }
    }

    pub fn view(&self) -> Element<SidebarMessage> {
        let topbar = row![
            bootstrap_icon(BootstrapIcon::Lamp),
            horizontal_space(),
            bootstrap_icon(BootstrapIcon::Gear),
        ];

        let content = column![
            topbar,
            self.children.create_commit.view().map(SidebarMessage::CreateCommitMessage),
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
