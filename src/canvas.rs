use std::collections::HashMap;
use std::cell::RefCell;
use std::io;
use std::io::{stdout, Stdout, Write};
use std::{thread, time};

use termion::{clear, color, cursor, style, terminal_size};
use termion::raw::{IntoRawMode, RawTerminal};
use num_complex::Complex;

pub struct Canvas {
    columns: i32,
    rows: i32,
    sleep_ms: u64,
    stdout: RefCell<RawTerminal<Stdout>>,
}

impl Canvas {
    pub fn new(sleep_ms: u64) -> io::Result<Self> {
        let size = terminal_size()?;
        let (columns, rows) = (i32::from(size.0), i32::from(size.1 - 1));

        let stdout = stdout();
        let mut stdout = stdout.into_raw_mode()?;
        write!(
            stdout,
            "{}{}{}{}",
            clear::All,
            cursor::Hide,
            color::Bg(color::White),
            color::Fg(color::Black),
        )?;
        for row in 1..(rows + 1) {
            write!(
                stdout,
                "{}{}",
                cursor::Goto(1, row as u16),
                " ".repeat(columns as usize)
            )?;
        }
        stdout.flush()?;
        Ok(Canvas {
            columns,
            rows,
            sleep_ms,
            stdout: RefCell::new(stdout),
        })
    }

    pub fn draw(
        &self,
        board: &HashMap<Complex<i32>, usize>,
        ant_position: Complex<i32>,
        ant_direction: Complex<i32>,
    ) -> io::Result<()> {
        let cell_ant = self.complex_to_screen(ant_position);
        let cell_prev = self.complex_to_screen(ant_position - ant_direction);

        // Draw the cell where the ant was (to remove the red color)
        if cell_ant != cell_prev {
            self.draw_cell(cell_prev, ant_position, board)?;
        }
        // Draw the cell with the ant
        self.draw_cell(cell_ant, ant_position, board)?;

        Ok(())
    }

    pub fn close(&self) -> io::Result<()> {
        let mut out = self.stdout.borrow_mut();
        write!(
            out,
            "{}{}{}",
            cursor::Goto(1, (self.rows + 1) as u16),
            style::Reset,
            cursor::Show
        )?;
        out.flush()?;

        Ok(())
    }

    fn draw_cell(
        &self,
        cell_location: (i32, i32),
        ant_position: Complex<i32>,
        board: &HashMap<Complex<i32>, usize>,
    ) -> io::Result<()> {
        if !((1 <= cell_location.0) && (cell_location.0 <= self.columns) && (1 <= cell_location.1)
            && (cell_location.1 <= self.rows))
        {
            return Ok(());
        }

        let (top, bottom) = self.screen_to_complex(cell_location.0, cell_location.1);
        let (fg, bg): (&color::Color, &color::Color) = (
            self.square_term_color(ant_position, top, board),
            self.square_term_color(ant_position, bottom, board),
        );

        let mut out = self.stdout.borrow_mut();
        write!(
            out,
            "{}{}{}{}",
            cursor::Goto(cell_location.0 as u16, cell_location.1 as u16),
            color::Fg(fg),
            color::Bg(bg),
            '▀'
        )?;
        out.flush()?;

        // If the ant is visible and we are asked to sleep then we sleep.
        if (ant_position == top || ant_position == bottom) && self.sleep_ms != 0 {
            thread::sleep(time::Duration::from_millis(self.sleep_ms));
        }

        Ok(())
    }

    fn square_term_color(
        &self,
        ant_position: Complex<i32>,
        square_position: Complex<i32>,
        board: &HashMap<Complex<i32>, usize>,
    ) -> &color::Color {
        if ant_position == square_position {
            return &color::Black;
        }
        let square_color = board.get(&square_position).cloned().unwrap_or(0);
        match square_color {
            0  => &color::White,
            1  => &color::Rgb(255,255,153),
            2  => &color::Rgb(31,120,180),
            3  => &color::Rgb(177,89,40),
            4  => &color::Rgb(51,160,44),
            5  => &color::Rgb(251,154,153),
            6  => &color::Rgb(227,26,28),
            7  => &color::Rgb(253,191,111),
            8  => &color::Rgb(255,127,0),
            9  => &color::Rgb(202,178,214),
            10 => &color::Rgb(106,61,154),
            11 => &color::Rgb(166,206,227),
            12 => &color::Rgb(178,223,138),
            _  => &color::Black,
        }
    }

    fn screen_to_complex(&self, column: i32, row: i32) -> (Complex<i32>, Complex<i32>) {
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
