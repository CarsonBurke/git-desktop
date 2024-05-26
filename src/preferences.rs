use iced::{widget::{column, container}, window, Element};

#[derive(Debug, Clone)]
pub enum PreferencesMessage {

}

#[derive(Debug, Clone)]
pub struct SidebarPreferences {
    pub min_width: u16,
    pub max_width: u16,
}

#[derive(Debug, Clone)]
pub struct Preferences {
    pub sidebar: SidebarPreferences,
}

impl Preferences {
    pub fn new() -> Self {
        Self {
            sidebar: SidebarPreferences {
                min_width: 200,
                max_width: 800,
            },
        }
    }

    pub fn view(&self) -> Element<Preferences> {
        
        let content = column![];
        let container = container(content);
        container.into()
    }
}