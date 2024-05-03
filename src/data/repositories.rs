use std::collections::HashMap;

pub type Repositories = HashMap<String, Repository>;

pub struct Repository {
    branches: HashMap<String, Branch>,
    current_branch: Option<String>,
}

impl Repository {
    pub fn new() -> Self {
        let mut new_self = Self {
            branches: HashMap::new(),
            current_branch: Some("main".to_string()),
        };

        new_self.branches.insert(
            "main".to_string(),
            Branch {
                name: "main".to_string(),
                commits: HashMap::new(),
            },
        );

        match new_self.branches.get_mut("main") {
            Some(branch) => {
                branch.commits.insert(
                    "id".to_string(),
                    Commit {
                        id: "id".to_string(),
                        author: "Carson".to_string(),
                        message: "commit message".to_string(),
                        time: 16,
                    },
                );
            }
            None => {}
        };

        new_self
    }
}

pub struct Branch {
    name: String,
    commits: HashMap<String, Commit>,
}

pub struct Commit {
    id: String,
    author: String,
    message: String,
    time: u16,
}
