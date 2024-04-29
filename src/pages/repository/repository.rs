use iced::{advanced::graphics::text::Version, widget::{column, container, row, text}, Element};
use iced_aw::BootstrapIcon;

use crate::general_widgets::icons::bootstrap_icon;

use super::version_graph::{VersionGraph, VersionGraphMessage};


#[derive(Debug, Clone, Copy)]
pub enum RepositoryPageMessage {
    VersionGraphMessage(VersionGraphMessage)
}

pub struct RepositoryPage {
    test_subpage: VersionGraph
}

impl RepositoryPage {
    pub fn new() -> Self {
        Self {
            test_subpage: VersionGraph::new()
        }
    }

    pub fn view(&self) -> Element<RepositoryPageMessage> {

        let content = column![
            self.test_subpage.view().map(move |message| {
                RepositoryPageMessage::VersionGraphMessage(message)
            })
        ];
        let container = container(content);
        container.into()
    }
}