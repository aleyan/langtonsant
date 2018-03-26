use std::collections::HashMap;
use std::io::{Write, stdout};
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
        ant_position: Complex<i32>, ant_direction: Complex<i32>){
        let stdout = stdout();
        let mut stdout = stdout.lock().into_raw_mode().unwrap();
        let white: Complex<i32> = Complex::new(0, -1);

        let (ant_column, ant_row) = self.complex_to_screen(ant_position);

        // Draw over the cell ant has left
        let (prev_column, prev_row) = self.complex_to_screen(
            ant_position - ant_direction);
        if (prev_column, prev_row) != (ant_column, ant_row)
            && (1 <= prev_column) && (prev_column <= self.columns)
            && (1 <= prev_row) && (prev_row <= self.rows) {
            let (top, bottom) = self.screen_to_complex(
                prev_column, prev_row);
            let top_color = board.get(&top).cloned().unwrap_or(white);
            let bottom_color = board.get(&bottom).cloned().unwrap_or(white);
            let symbol = if top_color == white && bottom_color == white {
                ' '
            } else if top_color != white && bottom_color == white {
                '▀'
            } else if top_color == white && bottom_color != white {
                '▄'
            } else if top_color != white && bottom_color != white {
                '█'
            }else {
                'X'
            };
            write!(stdout, "{}{}{}{}",
                cursor::Goto(prev_column as u16, prev_row as u16),
                color::Fg(color::Black),
                color::Bg(color::White),
                symbol
                ).unwrap();
        }

        // Draw the ant
        if (1 <= ant_column) && (ant_column <= self.columns)
            && (1 <= ant_row) && (ant_row <= self.rows) {
            let (top, bottom) = self.screen_to_complex(
                ant_column, ant_row);
            let top_color = board.get(&top).cloned().unwrap_or(white);
            let bottom_color = board.get(&bottom).cloned().unwrap_or(white);
            let (symbol, bg_is_white) = 
                if top == ant_position && bottom_color == white {
                    ('▀', true)
                } else if top == ant_position && bottom_color != white {
                    ('▀', false)
                } else if bottom == ant_position && top_color == white {
                    ('▄', true)
                } else if bottom == ant_position && top_color != white {
                    ('▄', false)
                } else {
                    ('█', true)
                };

            if bg_is_white {
                write!(stdout, "{}{}{}{}",
                    cursor::Goto(ant_column as u16, ant_row as u16),
                    color::Fg(color::Red),
                    color::Bg(color::White),
                    symbol
                    ).unwrap();
            }  else {
                write!(stdout, "{}{}{}{}",
                    cursor::Goto(ant_column as u16, ant_row as u16),
                    color::Fg(color::Red),
                    color::Bg(color::Black),
                    symbol
                    ).unwrap();
            };
            thread::sleep(time::Duration::from_millis(1));
        }

        stdout.flush().unwrap();
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

    fn screen_to_complex(&self, column: i32, row: i32)
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
        (column, row)
    }
 }
