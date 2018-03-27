extern crate num_complex;
extern crate termion;
#[macro_use]
extern crate clap;

use std::collections::HashMap;
use num_complex::Complex;
use clap::{App, Arg};

mod canvas;

fn main() {
    let matches = App::new("Langton's Ant")
       .version(crate_version!())
       .about("Simulates Langton's Ant in the terminal")
       .author("Alex Yankov")
       .arg(Arg::with_name("sleep")
            .short("s")
            .long("sleep")
            .value_name("MILLISECONDS")
            .help("Sets a custom sleep time between steps.")
            .takes_value(true))
       .arg(Arg::with_name("steps")
            .short("m")
            .long("maxsteps")
            .value_name("STEPS")
            .help("Maximum number of steps to simulate before terminating.")
            .takes_value(true))
       .get_matches();
    let sleep_ms = value_t!(matches, "sleep", u64).unwrap_or(0);
    let max_steps = value_t!(matches, "steps", u64).unwrap_or(15000);

    let canvas = canvas::Canvas::new(sleep_ms);

    let white: Complex<i32> = Complex::new(0, -1);

    let mut board: HashMap<Complex<i32>, Complex<i32>> = HashMap::new();
    let mut ant_position: Complex<i32> = Complex::new(0, 0);
    let mut ant_direction: Complex<i32> = Complex::new(-1, 0);

    for _ in 0..max_steps {
        let square_color = board.get(&ant_position).cloned().unwrap_or(white);
        ant_direction =  ant_direction * square_color;
        board.insert(ant_position, -square_color);
        ant_position = ant_position + ant_direction;

        canvas.draw(&board, ant_position, ant_direction);
    }
    canvas.close();
}
