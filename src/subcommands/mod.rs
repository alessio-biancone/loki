use std::error;

mod youtrack;

pub async fn handle(
    subcommand: Option<(&str, &clap::ArgMatches)>,
) -> Result<(), Box<dyn error::Error>> {
    match subcommand {
        Some(("youtrack", matches)) => youtrack::handle(matches).await,

        _ => unreachable!("what are you doing here?.."),
    }
}
