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
    ChangePage(MainChildPage),
    RepositoryPageMessage(RepositoryPageMessage),
}

#[derive(Debug, Clone)]
pub enum MainChildPage {
    RepositoryPage(RepositoryPage),
}

pub struct Main {
    page: MainChildPage,
}

impl Main {
    pub fn new() -> Self {
        Self {
            page: MainChildPage::RepositoryPage(RepositoryPage::new()),
        }
    }

    pub fn view(
        &self,
        repository_name: &Option<String>,
        repositories: &Repositories,
    ) -> Element<MainMessage> {
        let content = column![match &self.page {
            MainChildPage::RepositoryPage(page) => {
                page.view(repository_name, repositories)
                    .map(MainMessage::RepositoryPageMessage)
            }
        }];
        let container = container(content);
        container.into()
    }
}
