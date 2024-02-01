use crate::api::{fetch_contributors, fetch_repos};
use crate::config;
use crate::types::Config;
use confy;

use std::collections::HashSet;
use futures::future::join_all;

pub async fn get_stats() {
    let config: Config = load_creds();
    if config.token.is_none() || config.username.is_none() {
        println!(
            "Please set your GitHub username and token using the `{} set` command",
            config::APP_NAME
        );
        return;
    }

    let token = config.token.as_ref().unwrap();
    let username = config.username.as_ref().unwrap();

    println!("Because this data is expensive to compute, this tool can run for a while.\nPlease be patient. Don't spam or GitHub will get mad.\nExpect retries, since GitHub needs time to calculate this data.\n--------------------------------");
    
    let user_repos = fetch_repos(&token, &username).await;
    println!("Found {} repos...\n--------------------------------", user_repos.len());

    let user_repo_set: HashSet<String> = user_repos.iter().map(|r| r.name.clone()).collect();
    let mut handles = Vec::new();

    for repo_name in &user_repos {
        println!("Fetching data for {}", repo_name.name);
        let handle = fetch_contributors(&token, &repo_name.name, &username);
        handles.push(handle);
    }

    let results = join_all(handles).await;

    let mut repo_stats: Vec<(String, i64, i64, i64)> = Vec::new();
    let mut investigated_repos = HashSet::new();

    for result in results {
        if let Ok(repo_contributions) = result {
            investigated_repos.insert(repo_contributions.repo_name.clone());

            let (additions, deletions, commits) = repo_contributions.contributions.iter().fold((0, 0, 0), |acc, contribution| {
                let a = contribution.weeks.iter().map(|week| week.a).sum::<i64>();
                let d = contribution.weeks.iter().map(|week| week.d).sum::<i64>();
                let c = contribution.weeks.iter().map(|week| week.c).sum::<i64>();
                (acc.0 + a, acc.1 + d, acc.2 + c)
            });

            repo_stats.push((repo_contributions.repo_name.clone(), additions, deletions, commits));
        } else if let Err(e) = result {
            println!("Unexpected error occurred: {:?}", e);
        }
    }

    // sort
    repo_stats.sort_by(|a, b| a.1.cmp(&b.1));

    for (repo_name, additions, deletions, commits) in &repo_stats {
        println!("--------------------------------");
        println!("Repo: {}", repo_name);
        println!("Additions: {}", additions);
        println!("Deletions: {}", deletions);
        println!("Commits: {}", commits);
    }

    let uninvestigated_repos: HashSet<_> = user_repo_set.difference(&investigated_repos).collect();

    println!("------------ SUMMARY ------------");
    let total_additions: i64 = repo_stats.iter().map(|(_, additions, _, _)| additions).sum();
    let total_deletions: i64 = repo_stats.iter().map(|(_, _, deletions, _)| deletions).sum();
    let total_commits: i64 = repo_stats.iter().map(|(_, _, _, commits)| commits).sum();

    println!("Total commits: {}", total_commits);
    println!("Total additions: {}", total_additions);
    println!("Total deletions: {}", total_deletions);
    println!("Repos successfully investigated: {} / {}", investigated_repos.len(), user_repos.len());
    println!("Repos not investigated: {:?}", uninvestigated_repos);
    println!("--------------------------------");
}



pub fn set_username(username: &str) {
    let mut cfg: Config = confy::load(config::APP_NAME).unwrap_or_default();
    cfg.username = Some(username.to_owned());
    confy::store(config::APP_NAME, &cfg).expect("Failed to save config");
}

pub fn clear_username() {
    let mut cfg: Config = confy::load(config::APP_NAME).unwrap_or_default();
    cfg.username = None;
    confy::store(config::APP_NAME, &cfg).expect("Failed to save config");
}

pub fn set_token(token: &str) {
    let mut cfg: Config = confy::load(config::APP_NAME).unwrap_or_default();
    cfg.token = Some(token.to_owned());
    confy::store(config::APP_NAME, &cfg).expect("Failed to save config");
}

pub fn clear_token() {
    let mut cfg: Config = confy::load(config::APP_NAME).unwrap_or_default();
    cfg.token = None;
    confy::store(config::APP_NAME, &cfg).expect("Failed to save config");
}

pub fn check_creds() -> Config {
    let cfg: Config = confy::load(config::APP_NAME).unwrap_or_default();
    cfg
}

pub fn load_creds() -> Config {
    let cfg: Config = confy::load(config::APP_NAME).unwrap_or_default();
    cfg
}

pub fn example() {
    println!("{} clear", config::APP_NAME);
    println!("{} creds", config::APP_NAME);
    println!(
        "{} set --username <YOUR_NAME> --token <YOUR_TOKEN>",
        config::APP_NAME
    );
    println!("{} stats", config::APP_NAME);
    println!("Flags: --help, --version");
}
