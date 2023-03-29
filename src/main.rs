use std::{error, fmt::Display};

use clap::Command;

mod macros;
mod subcommands;

const NEWLINE: &str = "\r";
const FIELD_SEPARATOR: &str = "|";

// TODO: add issue type, may the gods be with you...
pub struct Issue {
    id: String,
    summary: String,
    description: String,
}

impl Display for Issue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{FIELD_SEPARATOR}{}{FIELD_SEPARATOR}{}",
            self.id, self.summary, self.description
        )
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let arguments = Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .bin_name(clap::crate_name!())
        .subcommand_required(true)
        .subcommand(
            clap::command!("youtrack")
                .arg(clap::arg!(--"token" <NAME>).required(true))
                .arg(clap::arg!(--"endpoint" <ENDPOINT>).required(true))
                .arg(clap::arg!(--"query" <QUERY>).required(true))
                .about("use youtrack issue getter"),
        )
        .get_matches();

    match subcommands::handle(arguments.subcommand()).await {
        Err(e) => {
            eprint!("{}", e);
            Ok(())
        }

        ok => ok,
    }
}
