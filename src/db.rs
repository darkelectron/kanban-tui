use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Debug)]
pub struct Board {
    pub id: i32,
    pub name: String
}

pub fn create_db() -> Result<()> {
    // Open a database connection
    // let conn = Connection::open("example.db")?;
    let conn = open_db()?;

    // Create a table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT UNIQUE NOT NULL
        )",
        [],
    )?;

    // Query the database
    let mut stmt = conn.prepare("SELECT id, name FROM cards")?;
    let user_iter = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;

    // println!("User: {} ({})", row.get(0), row.get(1));
    // println!("{:?}", user_iter);
    for user in user_iter {
        println!("{:?}", user.unwrap());
    }

    Ok(())
}

pub fn read_user_data(name: &str) -> Result<User> {
    let conn = open_db()?;
    // Query the database
    let mut stmt = conn.prepare("SELECT id, name FROM cards WHERE name = ?")?;
    let user_iter = stmt.query_row([name], |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;

    Ok(user_iter)
}

pub fn read_board_name(name: &str) -> Result<Board> {
    let conn = open_db()?;
    // Query the database
    let mut stmt = conn.prepare("SELECT id, name FROM boards WHERE name = ?")?;
    let board_name = stmt.query_row([name], |row| {
        Ok(Board {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;

    Ok(board_name)
}

pub fn insert_user_data(name: &str, email: &str) -> Result<usize, rusqlite::Error> {
    // Insert data into the table
    println!("inserting ");
    let conn = open_db()?;
    conn.execute(
        "INSERT INTO users (name, email) VALUES (?, ?)",
        (name, email)
    )
}

fn open_db() -> Result<Connection, rusqlite::Error> {
    Connection::open("example.db")
    // return conn;
}
