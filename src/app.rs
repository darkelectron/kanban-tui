use std::error;
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, PartialEq)]
pub enum AppMode {
    Main,
    CardEdit,
    ListEdit,
}

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub lists: Vec<Vec<String>>,
    pub row: usize,
    pub col: usize,
    pub mode: AppMode,
    pub prev_val: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            row: 0,
            col: 0,
            mode: AppMode::Main,
            prev_val: String::new(),
            lists: vec![
                vec![
                    String::from("Card 1 in List 1"),
                    String::from("Card 2 in List 1"),
                    String::from("Card 3 in List 1"),
                ],
                vec![String::from("Card 1 in List 2")],
                vec![String::from("Absolutly Nothing")],
            ],
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.running = false;
    }

    fn update_selection(&mut self) {
        self.col = self.col.min(self.lists.len().saturating_sub(1));
        self.row = self.row.min(self.lists[self.col].len().saturating_sub(1));
    }

    // Selection Motions
    pub fn motion_left(&mut self) {
        self.col = self.col.saturating_sub(1);
        self.update_selection();
    }
    pub fn motion_down(&mut self) {
        self.row += 1;
        self.update_selection();
    }
    pub fn motion_up(&mut self) {
        self.row = self.row.saturating_sub(1);
        self.update_selection();
    }
    pub fn motion_right(&mut self) {
        self.col += 1;
        self.update_selection();
    }

    // Card Movements
    pub fn move_left(&mut self) {
        if self.col == 0 || self.row >= self.lists[self.col].len() {
            return;
        }
        let t_col = self.col - 1;
        let t_row = self.row.min(self.lists[t_col].len());
        let card = self.lists[self.col].remove(self.row);
        self.lists[t_col].insert(t_row, card);
        self.row = t_row;
        self.col = t_col;
    }
    pub fn move_down(&mut self) {
        let t_row = self.row + 1;
        if t_row >= self.lists[self.col].len() {
            return;
        }
        self.lists[self.col].swap(self.row, t_row);
        self.row = t_row;
    }
    pub fn move_up(&mut self) {
        let t_row = self.row.saturating_sub(1);
        if t_row >= self.lists[self.col].len() {
            return;
        }
        self.lists[self.col].swap(self.row, t_row);
        self.row = t_row;
    }
    pub fn move_right(&mut self) {
        if self.row >= self.lists[self.col].len() {
            return;
        }
        let t_col = self.col + 1;
        if t_col >= self.lists.len() {
            return;
        }
        let t_row = self.row.min(self.lists[t_col].len());
        let card = self.lists[self.col].remove(self.row);
        self.lists[t_col].insert(t_row, card);
        self.row = t_row;
        self.col = t_col;
    }

    // Edit Card
    pub fn edit_card(&mut self) {
        if self.row >= self.lists[self.col].len() {
            return;
        }
        self.mode = AppMode::CardEdit;
        self.prev_val = self.lists[self.col][self.row].clone();
    }
    pub fn type_card(&mut self, c: char) {
        self.lists[self.col][self.row].push(c);
    }
    pub fn backspace_card(&mut self) {
        self.lists[self.col][self.row].pop();
    }
    pub fn done_editing(&mut self) {
        self.mode = AppMode::Main;
    }
    pub fn cancel_editing(&mut self) {
        self.lists[self.col][self.row] = self.prev_val.clone();
        self.mode = AppMode::Main;
    }

    // Add Card
    fn add_card(&mut self, index: usize) {
        self.lists[self.col].insert(index, String::from("New Card"));
        self.row = index;
        self.edit_card();
    }
    pub fn append_card(&mut self) {
        let index = (self.row + 1).min(self.lists[self.col].len());
        self.add_card(index);
    }
    pub fn prepend_card(&mut self) {
        self.add_card(self.row);
    }
    pub fn remove_card(&mut self) {
        if self.lists[self.col].is_empty() {
            return;
        }
        self.lists[self.col].remove(self.row);
        self.update_selection();
    }
}
