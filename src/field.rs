use cell::Cell;
use coord::Coord;

pub struct Field {
    cells: Vec<Vec<Cell>>,
    height: usize,
    width: usize,
}

impl Field {
    pub fn new(width: usize, height: usize, initial_coords: Option<Vec<Coord>>) -> Field {
        let initial_coords = initial_coords.unwrap_or(Vec::new());

        let mut rows: Vec<Vec<Cell>> = Vec::new();
        for y in 0..height {
            let mut row: Vec<Cell> = Vec::new();
            for x in 0..width {
                if contains_coord(&initial_coords, &Coord::new(x, y)) {
                    row.push(Cell::new(true))
                } else {
                    row.push(Cell::new(false))
                }
            }
            rows.push(row);
        }

        Field { cells: rows, height, width }
    }

    pub fn draw(&self) {
        for row in self.cells.iter() {
            for cell in row.iter() {
                print!("{}", if cell.is_alive() {'#'} else {' '});
            }
            println!{""};
        }
    }

    fn get_cell(&self, coord: &Coord) -> Option<&Cell> {
        let row: Option<&Vec<Cell>> = self.cells.get(coord.get_y());
        let cell: Option<&Cell> = match row {
            Some(row) => row.get(coord.get_x()),
            None => None,
        };
        cell
    }

    fn count_alive_neighbours(&self, coord: &Coord) -> usize {
        let neighbour_coords = get_neighbour_coords(
            coord,
            self.width,
            self.height,
        );

        // println!("for coord ({}, {}) neighbours are: ", coord.get_x(), coord.get_y());
        let mut count: usize = 0;        
        for coord in neighbour_coords.iter() {
            // println!("({}, {})", coord.get_x(), coord.get_y());

            if self.get_cell(coord)
                .expect("Attempting to get cell from invalid coordinates.")
                .is_alive() {
                count = count + 1;
            }
        }
        count
    }

    pub fn step(&mut self) {
        let mut x: usize = 0;
        let mut y: usize = 0;

        let mut new_rows: Vec<Vec<Cell>> = Vec::new();
        

        for row in self.cells.iter() {

            let mut new_cells: Vec<Cell> = Vec::new();

            for cell in row.iter() {
                let count = self.count_alive_neighbours(&Coord::new(x, y));

                let new_cell = if count < 2 {
                    Cell::new(false) // underpopulation
                } else if count == 3 {
                    Cell::new(true) // reproduction
                } else if count > 3 {
                    Cell::new(false) // overpopulation
                } else {
                    Cell::new(cell.is_alive())
                };

                new_cells.push(new_cell);

                x = x + 1;
            }

            new_rows.push(new_cells);

            y = y + 1;
            x = 0;
        }

        self.cells = new_rows;
    }
}




fn contains_coord(coords: &Vec<Coord>, coord: &Coord) -> bool {
    let mut contains: bool = false;

    for current_coord in coords.iter() {
        if current_coord == coord {
            contains = true;
            break;
        }
    }
    contains
}

fn get_neighbour_coords(coord: &Coord, width: usize, height: usize) -> Vec<Coord>
{
    let mut neighbours: Vec<Coord> = Vec::new();
    for i in 0..3 {
        for j in 0..3 {
            let (dx, dy): (isize, isize) = (i-1, j-1);

            let new_x: isize = coord.get_x() as isize + dx;
            let new_y: isize = coord.get_y() as isize + dy;

            if (new_x >= 0 && new_x < (width as isize)) && 
                (new_y >= 0 && new_y < (height as isize)) &&
                !(dx == 0 && dy == 0) {
                neighbours.push(Coord::new(new_x as usize, new_y as usize));
            }
        }
    }

    neighbours
}
