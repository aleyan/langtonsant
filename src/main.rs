#[macro_use]
extern crate clap;
extern crate termion;
extern crate palette;
extern crate nalgebra;

use clap::{App, Arg};

mod canvas;
mod simulator;

fn main() {
    let matches = App::new("Langton's Ant")
        .version(crate_version!())
        .about("Simulates Langton's Ant in the terminal")
        .author("Alexander Yankov")
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
                    "A sequence of states of cell that the ant advances by
one with every visit. The state of the cells rotates the
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



    let canvas = match canvas::Canvas::new(sleep_ms, fill_terminal, draw_ant, rotations.len()) {
        Ok(canvas) => canvas,
        Err(_) => {
            println!("Error acquiring stdout.");
            return;
        }
    };

    let mut sim = simulator::Simulator::new(rotations).unwrap();
    for _ in 0..max_steps {
        sim.simulate();
        let _ = canvas.draw(&sim.board, sim.ant_position, sim.ant_direction);
    }
}
