pub mod url;

use std::{str::FromStr, string::ParseError, process::{Command, ExitStatus}, io};


use std::io::{Write};
use clap::{Parser, Subcommand, command};

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
    Open {
        #[arg(short, long, default_value_t=true)]
        change_dir: bool ,

        #[arg(short, long)]
        jira_card_url: String,
    },
    Create {
        #[arg(short, long, default_value_t=true)]
        change_dir: bool ,

        #[arg(short, long)]
        jira_card_url: String,
    },
}


#[derive(Debug)]
struct JiraCard {
  pub card_number: String
}


impl JiraCard {
  fn go_to_card(&self) -> () {
      wait_for_command(Command::new("open")
      .arg("--url")
      .arg(
          format!("https://ailo.atlassian.net/browse/{}", 
                  self.card_number)));
  }

  fn create_worktree (&self) {
      wait_for_command(Command::new("git")
                .arg("worktree")
                .arg("add")
                .arg(&self.card_number));
      println!("worktree created at ./{}", &self.card_number);
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
        Commands::Open { jira_card_url, change_dir: _ } => {
            let card = jira_card_url.parse::<JiraCard>().unwrap();

            card.go_to_card();
        }

        Commands::Create { jira_card_url, change_dir: _ } => {
            let card = jira_card_url.parse::<JiraCard>().unwrap();

            card.create_worktree();
        }
    }
}

fn wait_for_command (cmd: &mut Command) -> ExitStatus {
    return cmd.spawn()
        .expect("This failed to call")
        .wait()
        .expect("This failed to wait");
}
