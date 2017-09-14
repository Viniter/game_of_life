mod cell;
mod coord;
mod field;

use coord::Coord;
use field::Field;
use std::{thread, time};

fn main() {
    let mut field = Field::new(
        30, 
        30, 
        Some(vec![
            Coord::new(15,15),
            Coord::new(15,16),
            Coord::new(16,14),
            Coord::new(16,15),
            Coord::new(17,15),
        ])
    );
    
    print!("{}[2J", 27 as char);
    loop {
        field.draw();
        thread::sleep(time::Duration::from_millis(250));
        print!("{}[2J", 27 as char);
        field.step();
    }
}
