use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Stylize},
    widgets::{block::Title, Block, List, ListItem, Paragraph},
    Frame,
};

use crate::app::{App, AppMode};

pub fn render(app: &App, frame: &mut Frame) {
    let rects = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(3),
    ])
    .split(frame.size());

    // Header
    frame.render_widget(
        Paragraph::new("Kanban TUI")
            .centered()
            .block(Block::bordered()),
        rects[0],
    );

    // Lists and cards
    let list_rects = Layout::horizontal(vec![Constraint::Length(40); app.lists.len()])
        .margin(1)
        .spacing(1)
        .split(rects[1]);
    app.lists.iter().enumerate().for_each(|(list_index, list)| {
        frame.render_widget(
            List::new(list.iter().enumerate().map(|(card_index, card)| {
                let is_selected = card_index == app.row && list_index == app.col;
                let mut text = card.clone();
                if is_selected && app.mode == AppMode::CardEdit {
                    text.push('_');
                }
                ListItem::new(text).fg(if is_selected {
                    Color::LightRed
                } else {
                    Color::White
                })
            }))
            .block(
                Block::bordered()
                    .title(Title::from(list_index.to_string()).alignment(Alignment::Left))
                    .title(Title::from(list.len().to_string()).alignment(Alignment::Right))
                    .fg(if list_index == app.col {
                        Color::LightRed
                    } else {
                        Color::White
                    }),
            ),
            list_rects[list_index],
        );
    });

    // Footer
    frame.render_widget(
        Paragraph::new(match app.mode {
            AppMode::Main => "Main",
            AppMode::CardEdit => "CardEdit",
            AppMode::ListEdit => "ListEdit",
        })
        .centered()
        .block(Block::bordered()),
        rects[2],
    );
}
