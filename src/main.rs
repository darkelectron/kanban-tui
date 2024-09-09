use kanban_tui::app::{App, AppResult};
use kanban_tui::event::{Event, EventHandler};
use kanban_tui::handler::handle_key_events;
use kanban_tui::tui::Tui;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
// use kanban_tui::db::{insert_user_data, read_user_data};

fn main() -> AppResult<()> {
    let mut app = App::new();
    // let _ = create_db();
    // let _ = insert_user_data("James", "htnoue");
    // let user_data = read_user_data("John");

    // match read_user_data("Jon") {
    //     Ok(user_data) => println!("{:?}", user_data),
    //     Err(_e) => {
    //
    //     }
    // }
    // for user in user_data {
    //     println!("{:?}", user);
    // }

    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    while app.running() {
        tui.draw(&app)?;
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    tui.exit()?;
    Ok(())
}
