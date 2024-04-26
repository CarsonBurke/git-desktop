use iced::{
    widget::{column, container, horizontal_space, row},
    Element, Length,
};
use iced_aw::BootstrapIcon;

use crate::{
    constants::spacings, general_widgets::icons::bootstrap_icon, styles::container::sidebar,
};

use super::history_and_changes::HistoryAndChanges;

#[derive(Debug)]
pub enum SidebarMessage {}

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
            }
        }
    }

    pub fn view(&self) -> Element<SidebarMessage> {
        let topbar = row![
            bootstrap_icon(BootstrapIcon::Lamp),
            horizontal_space(),
            bootstrap_icon(BootstrapIcon::Gear),
        ];

        let content = column![topbar];
        let container = container(content)
            .padding(spacings::SMALL)
            .height(Length::Fill)
            .width(Length::Shrink)
            .max_width(200)
            .style(sidebar());

        container.into()
    }
}
