extern crate num_complex;

use std::collections::HashMap;
use num_complex::Complex;

fn main() {
    let white: Complex<i32> = Complex::new(0, -1);

    let mut board: HashMap<Complex<i32>, Complex<i32>> = HashMap::new();
    let mut ant_position: Complex<i32> = Complex::new(0, 0);
    let mut ant_direction: Complex<i32> = Complex::new(0, 1); 

    loop {
        let square_color = board.get(&ant_position).cloned().unwrap_or(white);
        ant_direction = ant_direction * square_color;
        board.insert(ant_position, -1 * square_color);
        ant_position = ant_position + ant_direction;

        println!("{}", board.len());
    }
    
}
