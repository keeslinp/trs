use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    text::Spans,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};

use crate::{model::Post, model::PostView, state::State};
use anyhow::{bail, Result};

use snailquote::unescape;
use textwrap::wrap;

fn wrap_text<'a>(s: &'a str, rect: Rect) -> Vec<Spans<'a>> {
    unescape(s)
        .expect("failed to escape string")
        .lines()
        .map(|line| wrap(line, rect.width as usize - 7).into_iter())
        .flatten()
        .map(|line| Spans::from(Span::from(String::from(line))))
        .collect() // TODO: Less froms?
}

fn render_subreddit_view<B: Backend>(
    f: &mut Frame<B>,
    size: Rect,
    posts: &[Post],
    list_state: &mut ListState,
) {
    let items: Vec<ListItem> = posts
        .iter()
        .map(|p| {
            let mut spans = wrap_text(p.title.as_str(), size);
            spans.push(Spans::from(Span::from(format!(
                "     {} comments, {} upvotes",
                p.num_comments, p.up_votes
            ))));
            ListItem::new(spans)
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
    f.render_stateful_widget(list, size, list_state);
}

fn render_post_view(
    f: &mut Frame<impl Backend>,
    size: Rect,
    post_view: &PostView,
    list_state: &mut ListState,
) {
    let items: Vec<ListItem> = post_view
        .comments
        .iter()
        .map(|comment| {
            let mut spans = wrap_text(comment.body.as_str(), size);
            spans.push(Spans::from(Span::from(format!(
                "      {} upvotes",
                comment.up_votes
            ))));
            ListItem::new(spans)
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
    f.render_stateful_widget(list, size, list_state);
}

fn render_loading<B: Backend>(f: &mut Frame<B>) {
    let text = Paragraph::new(Span::from("Loading..."));
    f.render_widget(text, f.size());
}

fn render_select_subreddit(
    f: &mut Frame<impl Backend>,
    size: Rect,
    prompt: &str,
    stack_tail: &mut [State],
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Max(size.height - 1), Constraint::Max(1)])
        .split(size);

    let text = Paragraph::new(Spans::from(vec![Span::from("/r/"), Span::from(prompt)]));
    render_frame(f, chunks[0], stack_tail);
    f.render_widget(text, chunks[1]);
}

pub fn render_frame(f: &mut Frame<impl Backend>, size: Rect, state_stack: &mut [State]) {
    match state_stack {
        [.., State::Loading] => {
            render_loading(f);
        }
        [.., State::SubList(ref posts, ref mut list_state)] => {
            render_subreddit_view(f, size, posts, list_state);
        }
        [.., State::PostView(ref post_view, ref mut list_state)] => {
            render_post_view(f, size, post_view, list_state);
        }
        [old_stack @ .., State::SelectSubreddit(ref prompt)] => {
            render_select_subreddit(f, size, prompt, old_stack);
        }
        [] => {}
    };
}

pub fn render<B: Backend>(terminal: &mut Terminal<B>, state_stack: &mut [State]) -> Result<()> {
    terminal.draw(|f| {
        render_frame(f, f.size(), state_stack);
    })?;
    Ok(())
}
