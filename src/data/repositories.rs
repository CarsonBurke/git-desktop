use std::collections::HashMap;

pub type Repositories = HashMap<String, Repository>;

pub struct Repository {
    pub branches: Branches,
    pub current_branch: Option<String>,
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
                commits: HashMap::new(),
            },
        );

        match new_self.branches.get_mut("main") {
            Some(branch) => {
                branch.commits.insert(
                    "id".to_string(),
                    Commit {
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

pub type Branches = HashMap<String, Branch>;

pub struct Branch {
    commits: Commits,
}

pub type Commits = HashMap<String, Commit>;

pub struct Commit {
    author: String,
    message: String,
    time: u16,
}
