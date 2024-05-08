use iced::{widget::{column, container}, Element};

#[derive(Debug, Clone)]
pub enum PreferencesMessage {

}

#[derive(Debug, Clone)]
pub struct Preferences {

}

impl Preferences {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self) -> Element<Preferences> {
        
        let content = column![];
        let container = container(content);
        container.into()
    }
}