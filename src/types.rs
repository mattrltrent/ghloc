use std::fmt;

// make this public
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Repo {
    pub name: String,
    pub owner: Owner,
}

#[derive(Debug, Deserialize)]
pub struct Owner {
    // deserialize as login via serde
    #[serde(rename = "login")]
    pub username: String,
}

pub type Contributions = Vec<Contribution>;

// wrapper type for Contributions but with repo name field
#[derive(Debug, Deserialize)]
pub struct RepoContributions {
    pub repo_name: String,
    pub contributions: Contributions,
}

impl RepoContributions {
    pub fn new(repo_name: String, contributions: Contributions) -> Self {
        Self {
            repo_name: repo_name,
            contributions,
        }
    }
    pub fn empty(repo_name: String) -> Self {
        Self {
            repo_name: repo_name,
            contributions: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Contribution {
    pub total: i64,
    pub weeks: Vec<Week>,
    pub author: Author,
}

#[derive(Debug, Deserialize)]
pub struct Author {
    pub login: String, // we only care about the login
}

#[derive(Debug, Deserialize)]
pub struct Week {
    pub w: i64,
    pub a: i64,
    pub d: i64,
    pub c: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoSummary {
    pub login: String,
    pub name: String,
    pub total_commits: i64,
    pub total_additions: i64,
    pub total_deletions: i64,
}

impl fmt::Display for RepoSummary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "----------\nCommits: {}\nAdditions: {}\nDeletions: {}",
            self.total_commits, self.total_additions, self.total_deletions
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub username: Option<String>,
    pub token: Option<String>,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let username_display = self.username.as_deref().unwrap_or("Not set");
        let token_display = self
            .token
            .as_ref()
            .map(|token| "*".repeat(token.len()))
            .unwrap_or_else(|| "Not set".to_owned());
        write!(
            f,
            "Username: {}\nToken: {}",
            username_display, token_display
        )
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            username: None,
            token: None,
        }
    }
}
