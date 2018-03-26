use std::collections::HashMap;
use std::io::{Write, stdout};
use std::cmp::{min, max};
use std::{thread, time};

use termion::{terminal_size, style, clear, cursor, color};
use termion::raw::{IntoRawMode};
use num_complex::Complex;

pub(crate) struct Canvas {
    columns: i32,
    rows: i32,
}

impl Canvas {
    pub(crate) fn new() -> Self {
        let size = terminal_size().unwrap();
        let columns = size.0 as i32;
        let rows = (size.1 - 1) as i32;

        let stdout = stdout();
        let mut stdout = stdout.lock().into_raw_mode().unwrap();
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
        Canvas {
            columns: columns,
            rows: rows,
        }
    }

    pub(crate) fn draw(&self, board: &HashMap<Complex<i32>, Complex<i32>>, 
        ant_position: &Complex<i32>){
        let stdout = stdout();
        let mut stdout = stdout.lock().into_raw_mode().unwrap();
        let white: Complex<i32> = Complex::new(0, -1);

        let (ant_column, ant_row) = self.complex_to_screen(*ant_position);
        for row in max(1, ant_row-1)..min(ant_row + 1, self.rows + 1) {
            write!(stdout, "{}",
                cursor::Goto(max(1, ant_column - 1) as u16, row as u16),
                ).unwrap();
            for column in max(1, ant_column - 1)..min(ant_column + 1, self.columns + 1) {
                let (top, bottom) = self.screen_to_complex(row, column);
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
        thread::sleep(time::Duration::from_millis(1));
    }

    fn screen_to_complex(&self, row: i32, column: i32) 
        -> (Complex<i32>, Complex<i32>) {
        let x = column - self.columns / 2;
        let y = self.rows - row * 2;
        let top: Complex<i32> = Complex::new(x, y);
        let bottom: Complex<i32> = Complex::new(x, y - 1);
        (top, bottom)
    }

    fn complex_to_screen(&self, loc: Complex<i32>) -> (i32, i32) {
        let column = loc.re + self.columns / 2;
        let row = (-loc.im + self.rows) / 2;
        (column,row)
    }

    pub(crate) fn close(&self){
        let stdout = stdout();
        let mut stdout = stdout.lock().into_raw_mode().unwrap();
        write!(stdout, "{}{}{}",
            cursor::Goto(1, (self.rows + 1) as u16),
            style::Reset,
            cursor::Show
            ).unwrap();
    }
 }
