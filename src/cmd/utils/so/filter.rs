use serde::{de::DeserializeOwned, Deserialize, Serialize};

use reqwest::Client;

use tracing::{error, info};

/// Created with [`create`].
///
/// If there's a need for more fields, update the fields in [`create`],
/// run the function again and replace this constant with the newly generated
/// filter.
///
pub const FILTER: &str = "!GA6rnU)jp95BuY0.ZNgu2js9EcJVQ";

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

    fn as_form_string(&self) -> String {
        format!(
            "base={}&include={}&unsafe={}",
            urlencoding::encode(&self.base),
            urlencoding::encode(&self.include.join(";")),
            self.r#unsafe,
        )
    }
}

#[derive(Debug, Deserialize)]
struct Wrapper<T: DeserializeOwned> {
    quota_max: u32,
    quota_remaining: u32,
    has_more: bool,
    #[serde(bound(deserialize = "T: DeserializeOwned"))]
    items: Vec<T>,
}

#[derive(Debug, Deserialize)]
struct Filter {
    filter: String,
    filter_type: FilterType,
    included_fields: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
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
        .body(filter.as_form_string())
        .send()
        .await?;

    let status = response.status();

    let body = response.text().await?;

    if status.is_success() {
        let wrapper: Wrapper<Filter> = serde_json::from_str(&body)?;
        let filter = &wrapper.items[0];

        info!(?filter);

        println!("successfully created filter: {}", filter.filter);
    } else {
        error!(?status, body);
    }

    Ok(())
}