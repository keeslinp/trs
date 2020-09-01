use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    text::{Span},
    widgets::{Block, Borders, List, Paragraph, ListItem},
    Frame, Terminal,
};

use crate::{model::Post, state::{State, View}};
use anyhow::Result;

fn render_subreddit_view<B: Backend>(f: &mut Frame<B>, posts: &[Post]) {
    let items: Vec<ListItem> = posts
        .iter()
        .map(|p| ListItem::new(p.title.as_str()))
        .collect();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(80),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(f.size());
    let block = Block::default().title("Block").borders(Borders::ALL);
    let list = List::new(items);
    f.render_widget(list, block.inner(chunks[0]));
    f.render_widget(block, chunks[0]);
    let var_name = Block::default().title("Block 2").borders(Borders::ALL);
    let block = var_name;
    f.render_widget(block, chunks[1]);
}

fn render_loading<B: Backend>(f: &mut Frame<B>) {
    let text = Paragraph::new(Span::from("Loading..."));
    f.render_widget(text, f.size());
}

pub fn render<B: Backend>(terminal: &mut Terminal<B>, state: &State) -> Result<()> {
    terminal.clear()?;
    terminal.draw(|f| {
        match state.view_state {
            View::Loading => {
                render_loading(f);
            }
            View::SubList(_) => {}
        };
    })?;
    Ok(())
}
