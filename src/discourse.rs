use reqwest::Client;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, offset::Utc};

struct Page {
    name: String,
    url: String,
}

#[derive(Debug, Deserialize)]
struct TopicList {
    topics: Vec<Topic>,
}

#[derive(Debug, Deserialize)]
struct Topic {
    id: u64,
    title: String,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
struct Post {
    id: u64,
    username: String,
    cooked: String,
    raw: Option<String>,
}

pub async fn scrape(page: Page) -> Result<(), anyhow::Error> {
    let url = page.url;
    let name = page.name;

    let client = Client::new();

    Ok(())
}

