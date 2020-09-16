use crate::model::Post;
use anyhow::Result;
use serde_json::Value;

pub async fn get_posts(subreddit: Option<&str>) -> Result<Vec<Post>> {
    let subreddit_chunk = subreddit.map(|subreddit| format!("r/{}/", subreddit)).unwrap_or("".to_owned());
    let data: Value = surf::get(format!("https://www.reddit.com/{}best.json", subreddit_chunk))
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
            v.iter().filter_map(|p| {
                p.as_object().and_then(|v| v.get("data")).and_then(|p| {
                    Some(Post {
                        title: p.get("title")?.to_string(),
                        url: p.get("url")?.to_string(),
                    })
                })
            })
        })
        .unwrap()
        .collect())
}
