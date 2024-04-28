use iced::{widget::{column, container}, Element};

#[derive(Debug)]

pub enum CreateCommitMessage {

}

pub struct CreateCommit {

}

impl CreateCommit {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn view(&self) -> Element<CreateCommitMessage> {

        let content = column![];
        let container = container(content);
        container.into()
    }
}