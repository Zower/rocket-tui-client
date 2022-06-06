use std::io;

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::app::App;

pub fn draw(f: &mut Frame<CrosstermBackend<io::Stdout>>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(f.size());

    let block = Block::default()
        .title("Block")
        .borders(Borders::ALL)
        .style(Style::default().bg(app.color));

    f.render_widget(block, chunks[0]);

    if app.split {
        let items = app
            .todos
            .iter()
            .map(|t| ListItem::new(format!("{t:?}")))
            .collect::<Vec<_>>();

        let list = List::new(items)
            .block(Block::default().title("todos").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");

        let mut state = ListState::default();
        state.select(Some(2));

        f.render_stateful_widget(list, chunks[1], &mut state);
    }

    let block = Block::default()
        .title("Block 3")
        .style(Style::default().bg(app.color))
        .borders(Borders::ALL);

    f.render_widget(block, chunks[2]);
}
