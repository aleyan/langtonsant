extern crate num_complex;
extern crate termion;

use std::collections::HashMap;
use num_complex::Complex;
use termion::{terminal_size, style, clear, cursor, color};
use termion::raw::{IntoRawMode};
use std::io::{Write, stdout};
use std::cmp::{min, max};

fn main() {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let size = terminal_size().unwrap();
    let rows = (size.1 - 1) as i32;
    let columns = size.0 as i32;
    write!(stdout, "{}{}{}{}",
        clear::All,
        cursor::Hide,
        color::Bg(color::White),
        color::Fg(color::Black),
        ).unwrap();
    for row in 1..(rows+1){
        write!(stdout, "{}{}",
            cursor::Goto(1,row as u16),
            " ".repeat(columns as usize)).unwrap();
    }
    stdout.flush().unwrap();

    let white: Complex<i32> = Complex::new(0, -1);

    let mut board: HashMap<Complex<i32>, Complex<i32>> = HashMap::new();
    let mut ant_position: Complex<i32> = Complex::new(0, 0);
    let mut ant_direction: Complex<i32> = Complex::new(-1, 0);

    for _ in 0..15000 {
        let square_color = board.get(&ant_position).cloned().unwrap_or(white);
        ant_direction =  ant_direction * square_color;
        board.insert(ant_position, -square_color);
        ant_position = ant_position + ant_direction;

        //draw(&board);
        // draw begins here
        let (ant_column, ant_row) = complex_to_screen(rows, columns, ant_position);
        for row in max(1, ant_row-1)..min(ant_row + 1, rows + 1) {
            write!(stdout, "{}",
                cursor::Goto(max(1, ant_column - 1) as u16, row as u16),
                ).unwrap();
            for column in max(1, ant_column - 1)..min(ant_column + 1, columns + 1) {
                let (top, bottom) = screen_to_complex(rows, columns, row, column);
                let top_color = board.get(&top).cloned().unwrap_or(white);
                let bottom_color = board.get(&bottom).cloned().unwrap_or(white);
                let symbol = if top_color == white && bottom_color == white {
                    " "
                } else if top_color != white && bottom_color == white {
                    "▀"
                } else if top_color == white && bottom_color != white {
                    "▄"
                } else if top_color != white && bottom_color != white {
                    "█"
                }else {
                    "X"
                };
                write!(stdout, "{}", symbol).unwrap();
            }
        }
        stdout.flush().unwrap();
    }
    
    write!(stdout, "{}{}{}",
        cursor::Goto(1, (rows + 1) as u16),
        style::Reset,
        cursor::Show
        ).unwrap();
}

fn screen_to_complex(rows: i32, columns: i32, row: i32, column: i32) 
    -> (Complex<i32>, Complex<i32>) {
    let x = column - columns / 2;
    let y = (rows / 2 - row) * 2;
    let top: Complex<i32> = Complex::new(x, y);
    let bottom: Complex<i32> = Complex::new(x, y - 1);
    (top, bottom)
}

fn complex_to_screen(rows: i32, columns: i32, loc: Complex<i32>) -> (i32, i32) {
    let column = loc.re + columns / 2;
    let row = - loc.im / 2 + rows / 2;
    (column,row)
}

//fn draw(board: &HashMap<Complex<i32>, Complex<i32>>) {
//    let size = terminal_size().unwrap();
//    let rows = size.0 - 1;
//    let columns = size.1;
//    println!("{:?} {}", size, board.len());
//}
