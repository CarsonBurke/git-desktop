use iced::{widget::{column, container}, Element};


#[derive(Debug, Clone)]
pub enum FileMessage {

}

pub struct File {

}

impl File {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn view(&self) -> Element<FileMessage> {
        
        let content = column![];
        let container = container(content);
        container.into()
    }
}