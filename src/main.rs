use clap::{Arg, ArgAction, Command};
extern crate nalgebra;
extern crate palette;
extern crate termion;

mod canvas;
mod simulator;

fn main() {
    let matches = Command::new("Langton's Ant")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Simulates Langton's Ant in the terminal")
        .author("Alexander Yankov")
        .arg(
            Arg::new("sleep")
                .short('s')
                .long("sleep")
                .value_name("MILLISECONDS")
                .default_value("0")
                .help("Sets a custom sleep time between steps.")
                .value_parser(clap::value_parser!(u64)),
        )
        .arg(
            Arg::new("steps")
                .short('m')
                .long("maxsteps")
                .value_name("STEPS")
                .default_value("15000")
                .help("Maximum number of steps ant takes before stopping.")
                .value_parser(clap::value_parser!(u64)),
        )
        .arg(
            Arg::new("rotations")
                .short('r')
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
                ),
        )
        .arg(
            Arg::new("fillterminal")
                .short('f')
                .long("fillterminal")
                .action(ArgAction::SetTrue)
                .help("Fills entire terminal. Does not skip last line."),
        )
        .arg(
            Arg::new("invisibleant")
                .short('i')
                .long("invisibleant")
                .action(ArgAction::SetTrue)
                .help("Do not draw the ant."),
        )
        .get_matches();

    let sleep_ms = *matches.get_one::<u64>("sleep").unwrap();
    let max_steps = *matches.get_one::<u64>("steps").unwrap();
    let rotations = matches.get_one::<String>("rotations").unwrap();
    let fill_terminal = matches.get_flag("fillterminal");
    let draw_ant = !matches.get_flag("invisibleant");

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
