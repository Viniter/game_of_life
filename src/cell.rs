pub struct Cell {
    alive: bool,
}

impl Cell {
    pub fn new(alive: bool) -> Cell {
        Cell { alive }
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    pub fn set_alive(&mut self, val: bool) {
        self.alive = val;
    }
}