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
        let cell_ant = self.complex_to_screen(ant_position);
        let cell_prev = self.complex_to_screen(
            ant_position - ant_direction);

        // Draw the cell with the ant
        self.draw_cell(cell_ant, ant_position, board);
        // Draw the cell where the ant was (to remove the red color)
        if cell_ant != cell_prev {
            self.draw_cell(cell_prev, ant_position, board);
        }

        thread::sleep(time::Duration::from_millis(0));
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

    fn draw_cell(&self, cell_location: (i32, i32),
        ant_position: Complex<i32>,
        board: &HashMap<Complex<i32>, Complex<i32>>) {
        if !((1 <= cell_location.0) && (cell_location.0 <= self.columns)
            && (1 <= cell_location.1) && (cell_location.1 <= self.rows)){
            return
        }

        let (top, bottom) = self.screen_to_complex(
            cell_location.0, cell_location.1);
        let (fg, bg): (&color::Color, &color::Color) = 
            (self.square_term_color(ant_position, top, &board),
            self.square_term_color(ant_position, bottom, &board));

        let stdout = stdout();
        let mut stdout = stdout.lock().into_raw_mode().unwrap();
        write!(stdout, "{}{}{}{}",
            cursor::Goto(cell_location.0 as u16, cell_location.1 as u16),
            color::Fg(fg),
            color::Bg(bg),
            '▀'
            ).unwrap();
        stdout.flush().unwrap();
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
