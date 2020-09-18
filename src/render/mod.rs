use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    text::Spans,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};

use crate::{
    model::Post,
    model::PostView,
    state::{State, View},
};
use anyhow::Result;

fn render_subreddit_view<B: Backend>(f: &mut Frame<B>, posts: &[Post], list_state: &mut ListState) {
    let items: Vec<ListItem> = posts
        .iter()
        .map(|p| {
            ListItem::new(vec![
                Spans::from(Span::from(p.title.as_str())),
                Spans::from(Span::from(format!(
                    "     {} comments, {} upvotes",
                    p.num_comments, p.up_votes
                ))),
            ])
        })
        .collect();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(10)].as_ref())
        .split(f.size());
    let block = Block::default();
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");
    f.render_stateful_widget(list, block.inner(chunks[0]), list_state);
    f.render_widget(block, chunks[0]);
}

fn render_post_view(f: &mut Frame<impl Backend>, post_view: &PostView, list_state: &mut ListState) {
    let items: Vec<ListItem> = post_view
        .comments
        .iter()
        .map(|comment| {
            ListItem::new(vec![
                Spans::from(Span::from(comment.body.as_str())),
                Spans::from(Span::from(format!("      {} upvotes", comment.up_votes))),
            ])
        })
        .collect();
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("List"))
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");
    f.render_stateful_widget(list, f.size(), list_state);
}

fn render_loading<B: Backend>(f: &mut Frame<B>) {
    let text = Paragraph::new(Span::from("Loading..."));
    f.render_widget(text, f.size());
}

pub fn render<B: Backend>(terminal: &mut Terminal<B>, state: &mut State) -> Result<()> {
    terminal.draw(|f| {
        match state.view_state {
            View::Loading => {
                render_loading(f);
            }
            View::SubList(ref posts, ref mut list_state) => {
                render_subreddit_view(f, posts, list_state);
            }
            View::PostView(ref post_view, ref mut list_state) => {
                render_post_view(f, post_view, list_state);
            }
        };
    })?;
    Ok(())
}
