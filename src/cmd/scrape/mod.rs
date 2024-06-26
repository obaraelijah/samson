use futures::stream::{FuturesUnordered, TryStreamExt};

use std::fs::create_dir_all;
use std::str::FromStr;

mod discourse;

pub async fn scrape(pages: Vec<Page>) -> Result<(), anyhow::Error> {
    create_dir_all("scrape/")?;

    pages
        .into_iter()
        .map(|p| match p.ty {
            PageType::Discourse => discourse::scrape(p),
        })
        .collect::<FuturesUnordered<_>>()
        .try_collect()
        .await
}

#[derive(Debug, Clone)]
pub struct Page {
    pub name: String,
    pub url: String,
    pub ty: PageType,
}

impl FromStr for Page {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let &[name, url, ty] = s.split(';').collect::<Vec<&str>>().as_slice() else {
            return Err(anyhow::anyhow!(
                "expected page to have three parts separated by `;`".to_owned(),
            ));
        };

        Ok(Self {
            name: name.to_owned(),
            url: url.to_owned(),
            ty: PageType::from_str(ty)?,
        })
    }
}

#[derive(Debug, Clone)]
pub enum PageType {
    Discourse,
}

impl FromStr for PageType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "discourse" => Ok(Self::Discourse),
            _ => Err(anyhow::anyhow!(format!("unknown variant `{s}`"))),
        }
    }
}
