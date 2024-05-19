use crate::question::Question;

use chrono::{offset::Utc, DateTime};
use csv::{Reader, Writer};
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize};

use std::fs::remove_file;
use std::path::Path;

use regex::Regex;

use once_cell::sync::Lazy;

use tracing::info;

use std::time::Duration;
use tokio::task::spawn_blocking;
use tokio::time::sleep;

use super::Page;

static WAIT_SECONDS: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"Please retry again in (?P<s>\d{1,2}) seconds").unwrap());

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
            info!(forum = url, page = page, topic = i);

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

        let name = name.clone();
        spawn_blocking(move || create_temp_file(&name, page, &questions)).await??;

        page += 1;
    }

    spawn_blocking(move || combine_temp_files(&name)).await??;

    Ok(())
}

async fn get<T: DeserializeOwned>(client: &Client, url: &str) -> Result<T, anyhow::Error> {
    loop {
        let response = client.get(url).send().await?;

        let status = response.status();

        let body = response.text().await?;

        if status.as_u16() == 429 {
            let seconds: u64 = WAIT_SECONDS
                .captures(&body)
                .ok_or(anyhow::anyhow!("no capture found"))?
                .name("s")
                .ok_or(anyhow::anyhow!("no capture name `s` found"))?
                .as_str()
                .parse()?;

            info!("sleeping for {seconds} seconds");

            sleep(Duration::from_secs(seconds)).await;

            continue;
        }

        return Ok(serde_json::from_str(&body)?);
    }
}

fn create_temp_file(name: &str, page: u64, questions: &[Question]) -> Result<(), anyhow::Error> {
    let mut w = Writer::from_path(format!("scrape/{name}-{page}.csv"))?;

    for q in questions {
        w.serialize(q)?;
    }

    w.flush()?;

    Ok(())
}

fn combine_temp_files(name: &str) -> Result<(), anyhow::Error> {
    let mut page = 0;
    let mut questions: Vec<Question> = Vec::new();

    loop {
        let path = format!("scrape/{name}-{page}.csv");
        let path = Path::new(&path);

        if !path.exists() {
            break;
        }

        for question in Reader::from_path(path)?.deserialize() {
            questions.push(question?);
        }
        page += 1;
    }
    let mut w = Writer::from_path(format!("scrape/{name}.csv"))?;

    for q in questions.into_iter().rev() {
        w.serialize(q)?;
    }

    w.flush()?;

    delete_temp_files(name)?;

    Ok(())
}

fn delete_temp_files(name: &str) -> Result<(), anyhow::Error> {
    let mut page = 0;

    loop {
        let path = format!("scrape/{name}-{page}.csv");
        let path = Path::new(&path);

        if !path.exists() {
            break;
        }

        remove_file(path)?;

        page += 1;
    }

    Ok(())
}
