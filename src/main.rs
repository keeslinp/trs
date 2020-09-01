use anyhow::Result;
use std::io;
use termion::raw::IntoRawMode;
use tui::{backend::TermionBackend, Terminal};

mod model;
mod msg;
mod render;
mod state;
mod update;

use render::render;
use state::State;

#[async_std::main]
async fn main() -> Result<()> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let state = State::default();
    render(&mut terminal, &state);
    // let posts = get_posts("rust").await?;
    // let items: Vec<ListItem> = posts.iter().map(|p| ListItem::new(p.title.as_str())).collect();
    Ok(())
}
