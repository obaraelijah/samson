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
    post_stream: Option<PostStream>,
}

#[derive(Debug, Deserialize)]
struct PostStream {
    posts: Vec<Post>,
}

#[derive(Debug, Deserialize)]
struct Post {
    id: u64,
    username: String,
    cooked: String,
    raw: Option<String>,
}

struct  Question {
    title: String,
    body_raw: String,
    body_cooked: String,
    created: DateTime<Utc>,
    username: String,
    url: String,
    source_id: String,
}

pub async fn scrape(page: Page) -> Result<(), anyhow::Error> {
    let url = page.url;
    let name = page.name;

    let client = Client::new();

    let mut page = 0;

    loop {
        let mut questions: Vec<Question> = Vec::with_capacity(30);

        let latest_topics: LatestTopics = get(
            &client,
            &format!("{url}/latest.json?order=created&page={page}"),
        )
        .await?;
    
        let topics = latest_topics.topic_list.topics;
    
        if topics.is_empty() {
            break;
        }

        for (i, topic) in topics.into_iter().enumerate() {
            let topic_url = format!("{}/t/{}", url, topic.id);
            let topic: Topic = get(&client, &format!("{topic_url}.json")).await?;

            let post_id = topic
                .post_stream
                .ok_or(anyhow::anyhow!("`post_stream` empty"))?
                .posts[0]
                .id;
            let post: Post = get(&client, &format!("{url}/posts/{post_id}.json")).await?;

            let q = Question {
                title: topic.title,
                created: topic.created_at,
                username: post.username,
                body_cooked: post.cooked,
                body_raw: post.raw.ok_or(anyhow::anyhow!("`post.raw` field empty"))?,
                url: topic_url,
                source_id: topic.id.to_string(),
            };
            questions.push(q);
        }
    }

    Ok(())
}

async fn get<T: DeserializeOwned>(client: &Client, url: &str) -> Result<T, anyhow::Error> {
    let response = client.get(url).send().await?;
    let body = response.text().await?;

    return  Ok(serde_json::from_str(&body)?);
}
