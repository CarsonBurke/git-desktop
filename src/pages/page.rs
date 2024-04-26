use iced::{widget::{column, container}, Element};

#[derive(Debug)]
pub enum PageMessage {

}

pub struct Page {

}

impl Page {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn view(&self) -> Element<PageMessage> {

        let content = column![];
        let container = container(content);
        container.into()
    }
}