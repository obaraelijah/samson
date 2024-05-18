use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize};
use chrono::{DateTime, offset::Utc};

use super::Page;

#[derive(Debug, Deserialize)]
struct LatestTopics {
    topic_list: TopicList,
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

    loop {
        let latest_topics: LatestTopics = get(
            &client,
            &format!("{url}/latest.json?order=created&page={page}"),
        ).await?;
    
        let topics = latest_topics.topic_list.topics;
    
        if topics.is_empty() {
            break;
        }

        for (i, topic) in topics.into_iter().enumerate() {
            let topic_url = format!("{}/t/{}", url, topic.id);

            let topic: Topic = get(&client, &format!("{topic_url}.json")).await?;

        }
    }

    Ok(())
}

async fn get<T: DeserializeOwned>(client: &Client, url: &str) -> Result<T, anyhow::Error> {
    let response = client.get(url).send().await?;
    let body = response.text().await?;

    return  Ok(serde_json::from_str(&body)?);
}
