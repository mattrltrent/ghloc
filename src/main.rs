use clap::{App, Arg, SubCommand};
pub mod api;
pub mod config;
pub mod ops;
pub mod types;
use ops::{check_creds, clear_token, clear_username, example, get_stats, set_token, set_username};

#[tokio::main]
async fn main() {
    let matches = App::new(config::APP_NAME)
        .version(format!("v{}", env!("CARGO_PKG_VERSION")).as_str())
        .author("Matthew Trent • matthewtrent.me")
        .about("Fetches your total LOC and commits from all your GitHub repos\nFor info on how to get a GitHub token, check the README.md at: github.com/mattrltrent/ghloc")
        .subcommand(SubCommand::with_name("creds").about("Displays the current GitHub credentials"))
        .subcommand(SubCommand::with_name("example").about("Shows an example of usage"))
        .subcommand(
            SubCommand::with_name("token")
                .about("Shows how to get a GitHub token")
        )
        .subcommand(
            SubCommand::with_name("set")
                .about("Sets your GitHub credentials")
                .arg(
                    Arg::with_name("username")
                        .long("username")
                        .help("Sets the GitHub username")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("token")
                        .long("token")
                        .help("Sets the GitHub token")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("clear")
                .about("Clears configuration values")
                .arg(
                    Arg::with_name("username")
                        .long("username")
                        .help("Clears the GitHub username"),
                )
                .arg(
                    Arg::with_name("token")
                        .long("token")
                        .help("Clears the GitHub token"),
                ),
        )
        .subcommand(SubCommand::with_name("stats").about("Fetches GitHub stats"))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("set") {
        // only do if both set
        if matches.is_present("username") && matches.is_present("token") {
            set_username(matches.value_of("username").unwrap());
            set_token(matches.value_of("token").unwrap());
        } else {
            println!("Both username and token must be set at once\nTry: {} set --username <YOUR_NAME> --token <YOUR_TOKEN>", config::APP_NAME);
        }
    } else if let Some(_) = matches.subcommand_matches("example") {
        example();
    } else if let Some(_) = matches.subcommand_matches("clear") {
        clear_token();
        clear_username();
        println!("Cleared credentials.")
    } else if matches.subcommand_matches("stats").is_some() {
        get_stats().await;
    } else if matches.subcommand_matches("token").is_some() {
        println!("For info on how to get a GitHub token, check the README.md at: github.com/mattrltrent/ghloc")
    } else if matches.subcommand_matches("creds").is_some() {
        println!("{}", check_creds());
    } else {
        println!("Use the `--help` flag for more information, `{} example` for an example of usage, or `{} token to learn how to get your token`", config::APP_NAME, config::APP_NAME);
    }
}
