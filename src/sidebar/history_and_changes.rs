use iced::{widget::{column, container}, Element};

use super::sidebar::DisplayState;


#[derive(Debug)]
pub enum HistoryAndChangesMessage {

}

pub struct HistoryAndChanges {
    pub display_state: DisplayState
}

impl HistoryAndChanges {

    pub fn new() -> Self {
        Self {
            display_state: DisplayState::Visible,
        }
    }

    pub fn view(&self) -> Element<HistoryAndChangesMessage> {

        let content=  column![];
        let container = container(content);
        container.into()
    }
}