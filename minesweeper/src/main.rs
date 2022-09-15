use std::{collections::HashSet};

struct Minesweeper {
    height: i32,
    width: i32,
    num_mines: i32,
    mines: HashSet<(i32, i32)>,
    mines_found: HashSet<(i32, i32)>,
    board: Vec<Vec<bool>>,
}

impl Minesweeper {
    pub fn new(height: i32, width: i32, num_mines: i32) {

    }

    pub fn print() {

    }

    pub fn is_mine(cell: (i32, i32)) -> bool {

        return true
    }

    pub fn nearby_mines(cell: (i32, i32)) -> i32 {

        return 0;
    }

    pub fn won(&self) -> bool {

        return self.mines == self.mines_found;
    }

}

struct Sentence {
    cells: HashSet<(i32, i32)>,
    count: i32,
}

impl Sentence {
    pub fn new(cells: HashSet<(i32, i32)>, count: i32) {

    }

    pub fn known_mines() {

    }

    pub fn known_safes() {

    }

    pub fn mark_mine(cell: (i32, i32)) {

    }

    pub fn mark_safe(cell: (i32, i32)) {

    }
}

impl PartialEq for Sentence {
    fn eq (&self, other: &Self) -> bool {
        return self.cells == other.cells && self.count == other.count
    }
}

struct MinesweeperAI {
    height: i32,
    width: i32,
    mines: HashSet<(i32, i32)>,
    safes: HashSet<(i32, i32)>,
    knowledge: Vec<Sentence>,
}

impl MinesweeperAI {
    pub fn new(height: i32, width: i32) {

    }

    fn mark_mine(cell: (i32, i32)) {

    }

    fn mark_safe(cell: (i32, i32)) {

    }

    fn nearby_cells_undetermined(cell: (i32, i32), count: i32) {

    }

    fn print_knowledge() {

    }

    fn update_safes() {

    }

    fn update_mines() {

    }

    pub fn add_knowledge(cell: (i32, i32), count: i32) {

    }

    pub fn make_safe_move() {

    }

    pub fn make_random_move() {

    }

}


fn main() {
    println!("Hello, world!");
}
