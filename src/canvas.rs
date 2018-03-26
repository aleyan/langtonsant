use std::collections::HashMap;
use std::io::{Write, stdout};
use std::{thread, time};

use termion::{terminal_size, style, clear, cursor, color};
use termion::raw::{IntoRawMode};
use num_complex::Complex;

pub struct Canvas {
    columns: i32,
    rows: i32,
}

impl Canvas {
    pub fn new() -> Self {
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

    pub fn draw(&self, board: &HashMap<Complex<i32>, Complex<i32>>, 
        ant_position: Complex<i32>, ant_direction: Complex<i32>){
        let stdout = stdout();
        let mut stdout = stdout.lock().into_raw_mode().unwrap();
        let symbol = '▀';

        let cell_ant = self.complex_to_screen(ant_position);

        // Draw over the cell ant has left
        let cell_prev = self.complex_to_screen(
            ant_position - ant_direction);
        if cell_ant != cell_prev
            && (1 <= cell_prev.0) && (cell_prev.0 <= self.columns)
            && (1 <= cell_prev.1) && (cell_prev.1 <= self.rows) {
            let (fg, bg): (&color::Color, &color::Color) = self.cell_colors(
                cell_prev, ant_position, board);
            write!(stdout, "{}{}{}{}",
                cursor::Goto(cell_prev.0 as u16, cell_prev.1 as u16),
                color::Fg(fg),
                color::Bg(bg),
                symbol
                ).unwrap();
        }

        // Draw the ant
        if (1 <= cell_ant.0) && (cell_ant.0 <= self.columns)
            && (1 <= cell_ant.1) && (cell_ant.1 <= self.rows) {
            let (fg, bg): (&color::Color, &color::Color) = self.cell_colors(
                cell_ant, ant_position, board);
            write!(stdout, "{}{}{}{}",
                cursor::Goto(cell_ant.0 as u16, cell_ant.1 as u16),
                color::Fg(fg),
                color::Bg(bg),
                symbol
                ).unwrap();

            thread::sleep(time::Duration::from_millis(0));
        }



        stdout.flush().unwrap();
    }
    pub fn close(&self){
        let stdout = stdout();
        let mut stdout = stdout.lock().into_raw_mode().unwrap();
        write!(stdout, "{}{}{}",
            cursor::Goto(1, (self.rows + 1) as u16),
            style::Reset,
            cursor::Show
            ).unwrap();
    }
    fn cell_colors(&self, cell_location: (i32, i32),
        ant_position: Complex<i32>,
        board: &HashMap<Complex<i32>, Complex<i32>>) 
        -> (&color::Color, &color::Color) {
        let (top, bottom) = self.screen_to_complex(
            cell_location.0, cell_location.1);
        (self.square_term_color(ant_position, top, &board),
        self.square_term_color(ant_position, bottom, &board))
    }

    fn square_term_color(&self, ant_position: Complex<i32>,
        square_position: Complex<i32>,
        board: &HashMap<Complex<i32>, Complex<i32>>)
        -> &color::Color {
        let white: Complex<i32> = Complex::new(0, -1);
        let square_color = board.get(&square_position).cloned().unwrap_or(white);
        if ant_position == square_position {
            &color::Red
        } else if square_color == white {
            &color::White
        } else {
            &color::Black
        }
    }

    fn screen_to_complex(&self, column: i32, row: i32)
        -> (Complex<i32>, Complex<i32>) {
        let re = column - self.columns / 2;
        let im = self.rows - row * 2;
        let top: Complex<i32> = Complex::new(re, im);
        let bottom: Complex<i32> = Complex::new(re, im - 1);
        (top, bottom)
    }

    fn complex_to_screen(&self, loc: Complex<i32>) -> (i32, i32) {
        let column = loc.re + self.columns / 2;
        let row = (-loc.im + self.rows) / 2;
        (column, row)
    }
 }
