extern crate num_complex;
extern crate termion;

use std::collections::HashMap;
use num_complex::Complex;
use termion::{terminal_size, cursor, color};
use termion::raw::{IntoRawMode};
use std::io::{Write, stdout};

fn main() {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let size = terminal_size().unwrap();
    let rows = (size.1 - 1) as i32;
    let columns = size.0 as i32;

    let white: Complex<i32> = Complex::new(0, -1);

    let mut board: HashMap<Complex<i32>, Complex<i32>> = HashMap::new();
    let mut ant_position: Complex<i32> = Complex::new(0, 0);
    let mut ant_direction: Complex<i32> = Complex::new(0, 1); 

    for _ in 0..500 {
        let square_color = board.get(&ant_position).cloned().unwrap_or(white);
        ant_direction = ant_direction * square_color;
        board.insert(ant_position, -1 * square_color);
        ant_position = ant_position + ant_direction;

        //draw(&board);
        // draw begins here

        for row in 1..(rows + 1) {
            write!(stdout, "{}{}{}",
                cursor::Goto(1, row as u16),
                color::Bg(color::White),
                color::Fg(color::Black),
                ).unwrap();
            for column in 1..(columns + 1){
                let locs = screen_to_complex(rows, columns, row, column);
                let top_color = board.get(&locs.0).cloned().unwrap_or(white);
                let bottom_color = board.get(&locs.1).cloned().unwrap_or(white);
                let symbol = if top_color.im == -1 && bottom_color.im == -1 {
                    " "
                } else if top_color.im == 1 && bottom_color.im == -1 {
                    "▀"
                } else if top_color.im == -1 && bottom_color.im == 1 {
                    "▄"
                } else if top_color.im == 1 && bottom_color.im == 1 {
                    "█"
                }else {
                    "X"
                };
                write!(stdout, "{}", symbol).unwrap();
            }
        }
        stdout.flush().unwrap();
    }
    
    write!(stdout, "{}{}", cursor::Goto(1, (rows + 1) as u16),termion::style::Reset).unwrap();
}

fn screen_to_complex(rows: i32, columns: i32, row: i32, column: i32) 
    -> (Complex<i32>, Complex<i32>) {
    let x = column - columns / 2;
    let y = (row - rows / 2) * 2;
    let top: Complex<i32> = Complex::new(x, y);
    let bottom: Complex<i32> = Complex::new(x, y + 1);
    (top, bottom)
}

//fn draw(board: &HashMap<Complex<i32>, Complex<i32>>) {
//    let size = terminal_size().unwrap();
//    let rows = size.0 - 1;
//    let columns = size.1;
//    println!("{:?} {}", size, board.len());
//}
