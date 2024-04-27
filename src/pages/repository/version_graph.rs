use iced::{widget::{column, container, row, text}, Element};
use iced_aw::BootstrapIcon;

use crate::general_widgets::icons::bootstrap_icon;


#[derive(Debug)]
pub enum VersionGraphMessage {

}

pub struct VersionGraph {

}

impl VersionGraph {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn view(&self) -> Element<VersionGraphMessage> {

        let content = column![
            row![bootstrap_icon(BootstrapIcon::Code), text("Branch name")],
            row!["content"],
        ];
        let container = container(content);
        container.into()
    }
}