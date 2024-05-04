use iced::{
    widget::{column, container, horizontal_space, row},
    Element, Length,
};
use iced_aw::BootstrapIcon;

use crate::{
    constants::spacings, general_widgets::icons::bootstrap_icon, styles::container::sidebar,
};

use super::{changes::{Changes, ChangesMessage}, history_and_changes::HistoryAndChanges};

#[derive(Debug, Clone)]
pub enum SidebarMessage {
    TestMessage(ChangesMessage),
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
            },
            test_child: Changes::new(),
        }
    }

    pub fn view(&self) -> Element<SidebarMessage> {
        let topbar = row![
            bootstrap_icon(BootstrapIcon::Lamp),
            horizontal_space(),
            bootstrap_icon(BootstrapIcon::Gear),
        ];

        let content = column![topbar, self.test_child.view().map(move |message| {
            SidebarMessage::TestMessage(message)
        })].spacing(spacings::MEDIUM);
        let container = container(content)
            .padding(spacings::SMALL)
            .height(Length::Fill)
            .width(Length::Shrink)
            .max_width(200)
            .style(sidebar());

        container.into()
    }
}
