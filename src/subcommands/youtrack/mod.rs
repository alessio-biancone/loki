use std::error;

use crate::{get_arg, Issue, FIELD_SEPARATOR, NEWLINE};

mod api;

pub async fn handle(matches: &clap::ArgMatches) -> Result<(), Box<dyn error::Error>> {
    let token = get_arg!(matches, "token");
    let endpoint = get_arg!(matches, "endpoint");
    let query = get_arg!(matches, "query");

    let client = reqwest::Client::new();

    let url = reqwest::Url::parse_with_params(
        &format!("{}/issues", endpoint),
        [
            ("fields", "idReadable,description,summary"),
            ("query", &query),
        ],
    )?;

    client
        .get(url)
        .bearer_auth(token)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .send()
        .await?
        .json::<Vec<api::Issue>>()
        .await?
        .iter()
        .map(|issue| {
            Issue {
                id: issue.id_readable.clone(),
                summary: issue
                    .summary
                    .clone()
                    .unwrap_or_default()
                    .replace("\n", NEWLINE),
                description: issue
                    .description
                    .clone()
                    .unwrap_or_default()
                    .replace("\n", NEWLINE)
                    .replace("|", FIELD_SEPARATOR),
            }
            .to_string()
        })
        .for_each(|issue| println!("{}", issue));

    Ok(())
}
