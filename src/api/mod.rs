use serde_json::Value;
use anyhow::Result;
use crate::model::Post;

pub async fn get_posts(subreddit: &str) -> Result<Vec<Post>> {
    let data: Value = surf::get(format!("https://www.reddit.com/r/{}/hot.json", subreddit))
        .set_header("User-agent", "RTS 0.1")
        .recv_json()
        .await
        .unwrap(); //?;
    Ok(data
        .as_object()
        .and_then(|v| v.get("data"))
        .and_then(|v| v.as_object())
        .and_then(|v| v.get("children"))
        .and_then(|v| v.as_array())
        .map(|v| {
            v.iter().filter_map(|p| p.as_object().and_then(|v| v.get("data")).and_then(|p| {
                Some(Post {
                    title: p.get("title")?.to_string(),
                })
            }))
        })
        .unwrap()
        .collect())
}

