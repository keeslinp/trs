use crate::model::{Comment, Post, PostView};
use anyhow::{anyhow, Result};
use serde_json::Value;

pub async fn get_posts(subreddit: Option<&str>) -> Result<Vec<Post>> {
    let subreddit_chunk = subreddit
        .map(|subreddit| format!("r/{}/", subreddit))
        .unwrap_or("".to_owned());
    let data: Value = surf::get(format!(
        "https://www.reddit.com/{}best.json",
        subreddit_chunk
    ))
    .set_header("User-agent", "RTS 0.1")
    .recv_json()
    .await
    .map_err(|_| anyhow!("Failed to fetch posts for sub {:?}", subreddit))?;

    Ok(data["data"]["children"]
        .as_array()
        .map(|v| {
            v.iter().filter_map(|p| {
                p.as_object().and_then(|v| v.get("data")).and_then(|p| {
                    Some(Post {
                        title: p.get("title")?.to_string(),
                        url: p.get("url")?.to_string(),
                        num_comments: p.get("num_comments")?.as_u64()?,
                        up_votes: p.get("ups")?.as_i64()?,
                        permalink: p.get("permalink")?.to_string(),
                    })
                })
            })
        })
        .ok_or(anyhow!("Failed to parse post list"))?
        .collect())
}

fn parse_comments(raw_value: &Value) -> Result<Vec<Comment>> {
    raw_value["data"]["children"]
        .as_array()
        .map(|raw_comments| {
            raw_comments
                .iter()
                .filter_map(|child| child.get("data"))
                .filter_map(|raw_comment| {
                    Some(Comment {
                        body: raw_comment["body"].to_string(),
                        up_votes: raw_comment["ups"].as_i64()?,
                        replies: parse_comments(&raw_comment["replies"]).unwrap_or(Vec::new()),
                    })
                })
                .collect()
        })
        .ok_or(anyhow!("Failed to parse comments"))
}

pub async fn get_post_view(permalink: &str) -> Result<PostView> {
    let data: Value = surf::get(format!("https://www.reddit.com{}.json", permalink))
        .set_header("User-agent", "RTS 0.1")
        .recv_json()
        .await
        .map_err(|_| anyhow!("Failed to fetch post view {}", permalink))?;
    data.as_array()
        .and_then(|v| {
            let mut iter = v.iter();
            match (iter.next(), iter.next()) {
                (Some(raw_comments), None) => Some(PostView {
                    self_text: None,
                    comments: parse_comments(raw_comments).ok()?,
                }),
                (Some(post), Some(raw_comments)) => {
                    let post = post["data"]["children"][0]["data"].as_object();
                    Some(PostView {
                        self_text: post.and_then(|post| {
                            Some((post["selftext"].to_string(), post["ups"].as_i64()?))
                        }),
                        comments: parse_comments(raw_comments).ok()?,
                    })
                }
                _ => unreachable!(), // There should be at least one
            }
        })
        .ok_or(anyhow!("Failed to parse post"))
}
