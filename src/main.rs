pub mod url;

use std::{str::FromStr, string::ParseError, process::Command, os::unix::process::CommandExt};

use clap::{Parser, Subcommand};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// create a worktree from jira url
    Create {
        #[arg(short, long)]
        url: String,

        #[arg(short, long, default_value_t=true)]
        change_dir: bool 
    },
}


#[derive(Debug)]
struct JiraCard {
  pub card_number: String
}

impl JiraCard {
  fn go_to_card(&self) -> () {
      let command = format!("open 'https://ailo.atlassian.net/browse/{}'", self.card_number);
      println!("{}", command);
      Command::new(command).spawn().expect("faild to open");
      return; 
  }
}

impl FromStr for JiraCard {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url = s.parse::<url::Url>()?;
        let card_number = url.params()
            .get("selectedIssue")
            .unwrap_or(url
                     .routes
                     .last()
                     .expect("should have a route there"))
            .to_owned();

        Ok(JiraCard{ card_number })
    }
}

fn main() {
    let args = Cli::parse();
    let command = args.command.expect("To be valid command");

    match command {
        Commands::Create { url, change_dir: _ } => {
            let card = url.parse::<JiraCard>().unwrap();

            card.go_to_card();
        }
    }
}
