use iced::{
    advanced::graphics::text::Version,
    widget::{column, container, row, text},
    Command, Element,
};
use iced_aw::BootstrapIcon;

use crate::{data::repositories::Repositories, general_widgets::icons::bootstrap_icon};

use super::{
    file::{File, FileMessage},
    version_graph::{VersionGraph, VersionGraphMessage},
};

#[derive(Debug, Clone)]
pub enum RepositoryPageMessage {
    VersionGraphMessage(VersionGraphMessage),
    FileMessage(FileMessage),
    ShowFile(String),
}

#[derive(Debug, Clone)]
enum Subpage {
    VersionGraph(VersionGraph),
    File(File),
}

#[derive(Debug, Clone)]
pub struct RepositoryPage {
    test_subpage: VersionGraph,
    subpage: Subpage,
}

impl RepositoryPage {
    pub fn new() -> Self {
        Self {
            test_subpage: VersionGraph::new(),
            subpage: Subpage::VersionGraph(VersionGraph::new()),
        }
    }

    pub fn update(&mut self, message: RepositoryPageMessage) -> Command<RepositoryPageMessage> {
        match message {
            RepositoryPageMessage::VersionGraphMessage(version_graph_message) => Command::none(),
            RepositoryPageMessage::ShowFile(relative_path) => {
                self.subpage = Subpage::File(File::new());

                Command::none()
            }
            RepositoryPageMessage::FileMessage(file_message) => Command::none(),
        }
    }

    pub fn view(
        &self,
        repository_name: &Option<String>,
        repositories: &Repositories,
    ) -> Element<RepositoryPageMessage> {
        let Some(repository_name) = repository_name else {
            return container(row![]).into();
        };

        let Some(repository) = repositories.get(repository_name) else {
            return container(row![]).into();
        };

        let content = column![match &self.subpage {
            Subpage::VersionGraph(version_graph) => {
                container(
                    version_graph
                        .view(
                            repository
                                .current_branch
                                .clone()
                                .unwrap_or("[Error] no branch".to_string()),
                            &repository.branches,
                        )
                        .map(RepositoryPageMessage::VersionGraphMessage),
                )
            }
            Subpage::File(file) => {
                container(file.view(repository_name, repository).map(RepositoryPageMessage::FileMessage))
            }
        }];
        let container = container(content);
        container.into()
    }
}
