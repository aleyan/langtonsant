#[macro_use]
extern crate clap;
extern crate num_complex;
extern crate termion;

use std::collections::HashMap;
use num_complex::Complex;
use clap::{App, Arg};

mod canvas;

fn main() {
    let matches = App::new("Langton's Ant")
        .version(crate_version!())
        .about("Simulates Langton's Ant in the terminal")
        .author("Alex Yankov")
        .arg(
            Arg::with_name("sleep")
                .short("s")
                .long("sleep")
                .value_name("MILLISECONDS")
                .default_value("0")
                .help("Sets a custom sleep time between steps.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("steps")
                .short("m")
                .long("maxsteps")
                .value_name("STEPS")
                .default_value("15000")
                .help("Maximum number of steps ant takes before stopping.")
                .takes_value(true),
        )
        .get_matches();
    let sleep_ms = value_t!(matches, "sleep", u64).unwrap();
    let max_steps = value_t!(matches, "steps", u64).unwrap();

    let canvas = match canvas::Canvas::new(sleep_ms) {
        Ok(canvas) => canvas,
        Err(_) => {
            println!("Error acquiring stdout.");
            return;
        }
    };

    // We are going to be working on a complex plane where reals are the x
    // coordinate with positive reals representing right columns of the screen.
    // Imaginary component represents the y coordinate with positive values
    // representing higher rows of the screen.

    // Use a HashMap because it grows O(n) with number of steps taken by ant.
    let mut board: HashMap<Complex<i32>, Complex<i32>> = HashMap::new();
    let mut ant_position: Complex<i32> = Complex::new(0, 0); // Ant is at origin
    let mut ant_direction: Complex<i32> = Complex::new(-1, 0); // facing left.

    // White is -i because multiplying a complex number by -1i rotates it
    // 90 degrees clockwise. Black is +i because multiplying by it
    // rotates the complex coordinate by 90 degrees counter clockwise.
    let white: Complex<i32> = Complex::new(0, -1);

    for _ in 0..max_steps {
        // Get the color of the square under the ant. Default to white.
        let square_color = board.get(&ant_position).cloned().unwrap_or(white);
        ant_direction *= square_color; // Rotate by color.
        board.insert(ant_position, -square_color); // Flip color of the square.
        ant_position += ant_direction; // Move the ant by its direction.

        match canvas.draw(&board, ant_position, ant_direction) {
            Ok(_) => {}
            Err(_) => {
                continue;
            }
        }
    }
    canvas.close().unwrap();
}
