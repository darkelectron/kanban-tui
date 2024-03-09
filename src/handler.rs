use crate::app::{App, AppMode, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

pub fn handle_key_events(ev: KeyEvent, app: &mut App) -> AppResult<()> {
    if ev.kind != KeyEventKind::Press {
        return Ok(());
    }
    match app.mode() {
        AppMode::Main => match ev.code {
            // Card Movemenent
            // HJKL: Move card
            KeyCode::Char('H') | KeyCode::Left if ev.modifiers == KeyModifiers::SHIFT => {
                app.move_left();
            }
            KeyCode::Char('J') | KeyCode::Down if ev.modifiers == KeyModifiers::SHIFT => {
                app.move_down();
            }
            KeyCode::Char('K') | KeyCode::Up if ev.modifiers == KeyModifiers::SHIFT => {
                app.move_up();
            }
            KeyCode::Char('L') | KeyCode::Right if ev.modifiers == KeyModifiers::SHIFT => {
                app.move_right()
            }
            // Selection Motions
            // hjkl: Move cursor
            KeyCode::Char('h') | KeyCode::Left => app.motion_left(),
            KeyCode::Char('j') | KeyCode::Down => app.motion_down(),
            KeyCode::Char('k') | KeyCode::Up => app.motion_up(),
            KeyCode::Char('l') | KeyCode::Right => app.motion_right(),
            // CRUD Operations On Lists
            // AIEX: Append/Prepend/Edit/Delete a list
            KeyCode::Char('A') if ev.modifiers == KeyModifiers::SHIFT => app.append_list(),
            KeyCode::Char('I') if ev.modifiers == KeyModifiers::SHIFT => app.prepend_list(),
            KeyCode::Char('E') if ev.modifiers == KeyModifiers::SHIFT => app.edit_list(),
            KeyCode::Char('X') if ev.modifiers == KeyModifiers::SHIFT => app.remove_list(),
            // CRUD Operations on Cards
            // aiex: Append/Prepend/Edit/Delete a card
            KeyCode::Char('a') => app.append_card(),
            KeyCode::Char('i') => app.prepend_card(),
            KeyCode::Char('e') => app.edit_card(),
            KeyCode::Char('x') => app.remove_card(),
            // Exit
            KeyCode::Char('q') => app.quit(),
            KeyCode::Char('c' | 'C') if ev.modifiers == KeyModifiers::CONTROL => app.quit(),
            _ => {}
        },
        AppMode::CardEdit => match ev.code {
            KeyCode::Enter => app.done_editing(),
            KeyCode::Esc => app.cancel_card_edit(),
            KeyCode::Backspace => app.backspace_card(),
            KeyCode::Char(c) => app.type_card(c),
            _ => {}
        },
        AppMode::ListEdit => match ev.code {
            KeyCode::Enter => app.done_editing(),
            KeyCode::Esc => app.cancel_list_edit(),
            KeyCode::Backspace => app.backspace_list(),
            KeyCode::Char(c) => app.type_list(c),
            _ => {}
        },
    };
    Ok(())
}
