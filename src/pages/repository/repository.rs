use iced::{
    advanced::graphics::text::Version,
    widget::{column, container, row, text},
    Element,
};
use iced_aw::BootstrapIcon;

use crate::{data::repositories::Repositories, general_widgets::icons::bootstrap_icon};

use super::version_graph::{VersionGraph, VersionGraphMessage};

#[derive(Debug, Clone)]
pub enum RepositoryPageMessage {
    VersionGraphMessage(VersionGraphMessage),
}

pub struct RepositoryPage {
    test_subpage: VersionGraph,
}

impl RepositoryPage {
    pub fn new() -> Self {
        Self {
            test_subpage: VersionGraph::new(),
        }
    }

    pub fn view(
        &self,
        repository_name: &Option<String>,
        repositories: &Repositories,
    ) -> Element<RepositoryPageMessage> {

        let Some(repository_name) = repository_name else {
            return container(row![]).into()
        };

        let Some(repository) = repositories.get(repository_name) else {
            return container(row![]).into()
        };


        let content = column![self
            .test_subpage
            .view(repository.current_branch.clone().unwrap_or("[Error] no branch".to_string()), &repository.branches)
            .map(RepositoryPageMessage::VersionGraphMessage)];
        let container = container(content);
        container.into()
    }
}
