use iced::{widget::{column, container}, Element};


#[derive(Debug)]
pub enum BranchHistoryMessage {

}

pub struct BranchHistory {

}

impl BranchHistory {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn view(&self) -> Element<BranchHistoryMessage> {

        let content = column![];
        let container = container(content);
        container.into()
    }
}