use iced::{
    widget::{column, container},
    Element,
};

use crate::data::repositories::Repositories;

use super::repository::{
    repository::{RepositoryPage, RepositoryPageMessage},
    version_graph::{self, VersionGraph},
};

#[derive(Debug, Clone)]
pub enum MainMessage {
    RepositoryPageMessage(RepositoryPageMessage),
}

pub struct Main {
    test_page: RepositoryPage,
}

impl Main {
    pub fn new() -> Self {
        Self {
            test_page: RepositoryPage::new(),
        }
    }

    pub fn view(
        &self,
        repository_name: &Option<String>,
        repositories: &Repositories,
    ) -> Element<MainMessage> {
        let content = column![self
            .test_page
            .view(repository_name, repositories)
            .map(move |message| { MainMessage::RepositoryPageMessage(message) })];
        let container = container(content);
        container.into()
    }
}
