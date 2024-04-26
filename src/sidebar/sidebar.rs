use iced::{widget::{column, container, row}, Element};

#[derive(Debug)]
pub enum SidebarMessage {

}

pub struct Sidebar {

}

impl Sidebar {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn view(&self) -> Element<SidebarMessage> {

        let topbar = row![];

        let content = column![topbar];
        let container = container(content);

        container.into()
    } 
}