#[macro_use]
extern crate clap;
extern crate termion;
extern crate palette;
extern crate nalgebra;

use nalgebra::{Vector2, Matrix2};//, Vector3, Matrix3};
use std::collections::HashMap;
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
        .arg(
            Arg::with_name("rotations")
                .short("r")
                .long("rotations")
                .value_name("SEQUENCE")
                .default_value("RL")
                .help(
                    "A sequence of states of squares that the ant advances by
one with every visit. The state of the square rotates the
ant as encoded in this sequence. The first element of the
sequence initially covers the entire board. Valid elements:
R - Turn 90 degrees to the right
L - Turn 90 degrees to the left
U - Turn 180 degrees
N - No change",
                )
                .takes_value(true),
        )
        .arg(
            Arg::with_name("fillterminal")
                .short("f")
                .long("fillterminal")
                .help("Fills entire terminal. Does not skip last line."),
        )
        .arg(
            Arg::with_name("invisibleant")
                .short("i")
                .long("invisibleant")
                .help("Do not draw the ant."),
        )
        .get_matches();
    let sleep_ms = value_t!(matches, "sleep", u64).unwrap();
    let max_steps = value_t!(matches, "steps", u64).unwrap();
    let rotations = matches.value_of("rotations").unwrap();
    let fill_terminal = matches.is_present("fillterminal");
    let draw_ant = !matches.is_present("invisibleant");

    let mut states: Vec<Matrix2<i32>> = Vec::new();
    for c in rotations.chars() {
        let rotation = match c {
            'R' => Matrix2::new(0, 1,-1, 0),
            'L' => Matrix2::new(0, -1,1, 0),
            'U' => Matrix2::new(-1, 0,0, -1),
            'N' => Matrix2::new(1, 0,0, 1),
            _ => {
                println!("Error. Invalid rotation.");
                return;
            }
        };
        states.push(rotation);
    }
    let states = states.clone();

    let canvas = match canvas::Canvas::new(sleep_ms, fill_terminal, draw_ant, states.len()) {
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
    let mut board: HashMap<Vector2<i32>, usize> = HashMap::new();
    let mut ant_position: Vector2<i32> = Vector2::new(0, 0); // Ant is at origin
    let mut ant_direction: Vector2<i32> = Vector2::new(-1, 0); // facing left.

    for _ in 0..max_steps {
        // Get the color of the square under the ant. Default to white.
        let square_color = board.get(&ant_position).cloned().unwrap_or(0);
        // Rotate by the state of square.
        ant_direction = states[square_color] * ant_direction;
        //Advance the state of the square by 1, possible wrap to back to 0
        board.insert(ant_position, (square_color + 1) % states.len());
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
