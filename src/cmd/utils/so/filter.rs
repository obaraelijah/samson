use axum::http::response;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use reqwest::Client;

use tracing::{error, info};

/// Created with [`create`].
///
/// If there's a need for more fields, update the fields in [`create`],
/// run the function again and replace this constant with the newly generated
/// filter.
///

#[derive(Debug, Serialize)]
struct CreateFilter {
    base: String,
    include: Vec<String>,
    r#unsafe: bool,
}

impl CreateFilter {
    fn new<B: AsRef<str>, I: AsRef<str>>(base: B, include: Vec<I>, r#unsafe: bool) -> Self {
        Self { 
            base: base.as_ref().to_owned(), 
            include: include.into_iter().map(|i| i.as_ref().to_owned()).collect(),
            r#unsafe,
        }
    }
}

struct Filter {
    filter: String,
    filter_type: FilterType,
    included_fields: Vec<String>,
}

enum FilterType {
    Safe,
    Unsafe,
    Invalid,
}

pub async fn create() -> Result<(), anyhow::Error> {
    let client = Client::new();

    let url = "https://api.stackexchange.com/2.3/filters/create";

    let filter = CreateFilter::new(
        "none",
        vec![
            ".backoff",
            ".error_id",
            ".error_message",
            ".error_name",
            ".has_more",
            ".items",
            ".quota_max",
            ".quota_remaining",
            "question.body",
            "question.body_markdown",
            "question.closed_date",
            "question.creation_date",
            "question.link",
            "question.owner",
            "question.question_id",
            "question.title",
            "shallow_user.display_name",
            "shallow_user.user_id",
        ],
        false,
    );

    let response = client
        .post(url)
        .header("content-type", "application/x-www-form-urlencoded")
        .body(filter.as())
        .send()
        .await?;
    Ok(())

}
