use crate::config;
use crate::types::{Contributions, Repo, RepoContributions};
use reqwest::header;
use reqwest::{Client, StatusCode};
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::time::sleep;

pub async fn fetch_repos(github_pat: &str, username: &str) -> Vec<Repo> {
    let client = reqwest::Client::new();
    let mut all_repos = Vec::new();
    let mut page = 1;

    loop {
        let url = format!(
            "https://api.github.com/user/repos?type=all&per_page=100&page={}",
            page
        );
        let resp = client
            .get(&url)
            .header(header::AUTHORIZATION, format!("token {}", github_pat))
            .header(header::USER_AGENT, "request")
            .send()
            .await
            .expect("Failed to send request");

        // check if the response status is successful
        if resp.status().is_success() {
            let repos: Vec<Repo> = resp
                .json()
                .await
                .expect("Failed to parse JSON while fetching a repo chunk");

            // break the loop if no more repositories are returned
            if repos.is_empty() {
                break;
            }

            // ensure only the user's repos are included (basically blocks out their organizations' repos that aren't
            // contributed to by them)
            all_repos.extend(repos.into_iter().filter(|r| r.owner.username == username));
            page += 1; // Increment the page number for the next request
        } else {
            // handle non-successful responses, possibly with custom error handling
            println!("Failed to fetch repositories: {}", resp.status());
            break;
        }
    }

    all_repos
}

pub fn fetch_contributors(
    github_pat: &str,
    repo_name: &str,
    owner: &str,
) -> JoinHandle<RepoContributions> {
    let github_pat = github_pat.to_string();
    let repo_name = repo_name.to_string();
    let owner = owner.to_string();

    tokio::spawn(async move {
        let client = Client::new();
        let mut attempts: u64 = 0;

        loop {
            let response = client
                .get(format!(
                    "https://api.github.com/repos/{}/{}/stats/contributors",
                    owner, repo_name
                ))
                .header(header::AUTHORIZATION, format!("Bearer {}", &github_pat))
                .header(header::USER_AGENT, "request")
                .header("X-GitHub-Api-Version", "2022-11-28")
                .send()
                .await;

            match response {
                Ok(resp) => {
                    match resp.status() {
                        StatusCode::OK => {
                            let contributors: Contributions = match resp.json().await {
                                Ok(data) => data,
                                Err(err) => {
                                    eprintln!(
                                        "Failed to parse contributors for {}, err: {}",
                                        repo_name, err
                                    );
                                    Vec::new()
                                }
                            };
                            return RepoContributions::new(repo_name, contributors);
                        }
                        StatusCode::ACCEPTED => {
                            // Data is being prepared, increase delay with each attempt
                            println!("Statistics for {} are being prepared, retrying after {} seconds...", repo_name, config::RETRY_DELAY * (attempts + 1));
                            attempts += 1;
                            sleep(Duration::from_secs(config::RETRY_DELAY * attempts)).await;
                        }
                        StatusCode::NO_CONTENT => {
                            println!("No content available for {}", repo_name);
                            return RepoContributions::empty(repo_name);
                        }
                        StatusCode::FORBIDDEN => {
                            println!("Access forbidden for {}. Check your token permissions or rate limits.", repo_name);
                            return RepoContributions::empty(repo_name);
                        }
                        _ => {
                            // Other errors, retry after a delay
                            println!(
                                "Failed to fetch contributors for {}. Retrying after {} seconds...",
                                repo_name,
                                config::RETRY_DELAY * (attempts + 1)
                            );
                            attempts += 1;
                            sleep(Duration::from_secs(config::RETRY_DELAY * attempts)).await;
                        }
                    }
                }
                Err(err) => {
                    // network or other errors, retry after a delay
                    println!(
                        "Failed to fetch contributors for {}: {}. Retrying after {} seconds...",
                        repo_name,
                        err,
                        config::RETRY_DELAY * (attempts + 1)
                    );
                    attempts += 1;
                    sleep(Duration::from_secs(config::RETRY_DELAY * attempts)).await;
                }
            }

            if attempts >= config::MAX_RETRIES as u64 {
                println!(
                    "Failed to fetch contributors for {} after {} retries.",
                    repo_name,
                    config::MAX_RETRIES
                );
                return RepoContributions::empty(repo_name);
            }
        }
    })
}
