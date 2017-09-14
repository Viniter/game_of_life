pub struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    pub fn new(x: usize, y: usize) -> Coord {
        Coord {x, y}
    }

    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> usize {
        self.y
    }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Coord) -> bool {
        self.x == other.x && self.y == other.y
    }
}
