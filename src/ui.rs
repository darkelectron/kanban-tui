use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Stylize},
    widgets::{block::Title, Block, List, ListItem, Paragraph},
    Frame,
};

use crate::{app::{App, AppMode}, db::read_board_name, db::Board};

pub fn render(app: &App, frame: &mut Frame) {
    let rects = Layout::vertical([
        Constraint::Length(3),
        Constraint::Fill(1),
        Constraint::Length(3),
    ])
    .split(frame.size());


    let board_name = match read_board_name("Default") {

        Ok(user_data) => user_data,

        Err(e) => {

            eprintln!("Error reading user data: {}", e);

            // Try to read the user data again or return a default user

            match read_board_name("Jon") {

                Ok(user_data) => user_data,

                Err(e) => {

                    eprintln!("Failed to read user data again: {}", e);

                    Board { id: 0, name: String::new() }

                }
            }
        }
    };

    // Header
    frame.render_widget(
        Paragraph::new("Kanban 🦀 TUI")
            .left_aligned()
            .block(Block::bordered()),
        rects[0],
    );

    let name = format!("📋️ {}", board_name.name.to_string());

    frame.render_widget(
        Paragraph::new(name)
            .centered()
            .block(Block::bordered()),
        rects[0],
    );

    // Lists and cards
    // TODO: Slice lists
    let list_rects = Layout::horizontal(vec![Constraint::Percentage(20); app.lists().len()])
        .margin(1)
        .spacing(1)
        .split(rects[1]);
    app.lists()
        .iter()
        .enumerate()
        .for_each(|(list_index, list)| {
            let list_title = if app.col() == list_index && app.mode() == AppMode::ListEdit {
                format!("{}_ ", list.name())
            } else {
                format!("{} ", list.name())
            };
            frame.render_widget(
                List::new(list.cards().iter().enumerate().map(|(card_index, card)| {
                    let is_selected = card_index == app.row() && list_index == app.col();
                    let mut text = card.clone();
                    if is_selected && app.mode() == AppMode::CardEdit {
                        text.push('_');
                    }
                    ListItem::new(text).fg(if is_selected {
                        Color::Green
                    } else {
                        Color::White
                    })
                })).highlight_symbol(">>")
                .block(
                    Block::bordered()
                        .title(Title::from(list_title).alignment(Alignment::Left))
                        .title(Title::from(list.len().to_string()).alignment(Alignment::Right))
                        .fg(if list_index == app.col() {
                            Color::LightBlue
                        } else {
                            Color::White
                        }),
                ).highlight_symbol(">>")
                .repeat_highlight_symbol(true),
                list_rects[list_index],
            );
        });

    // Footer
    // TODO: Keybindings for each mode
    frame.render_widget(
        Paragraph::new(match app.mode() {
            AppMode::Main => "Main",
            AppMode::CardEdit => "CardEdit",
            AppMode::ListEdit => "ListEdit",
        })
        .centered()
        .block(Block::bordered()),
        rects[2],
    );
}
