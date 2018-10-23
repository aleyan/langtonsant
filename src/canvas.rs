use std::collections::HashMap;
use std::cell::RefCell;
use std::io;
use std::io::{stdout, Stdout, Write};
use std::{thread, time};

use termion::{clear, color, cursor, style, terminal_size};
use termion::raw::{IntoRawMode, RawTerminal};
use nalgebra::{Vector2};
use palette::{Srgb, LinSrgb, Lch, Gradient, Shade};

pub struct Canvas {
    columns: i32,
    rows: i32,
    sleep_ms: u64,
    draw_ant: bool,
    colors: Vec<(u8, u8, u8)>,
    stdout: RefCell<RawTerminal<Stdout>>,
}

impl Canvas {
    pub fn new(sleep_ms: u64, fill_terminal: bool, draw_ant: bool, number_of_states: usize) -> io::Result<Self> {
        let size = terminal_size()?;

        let (columns, rows) = match fill_terminal {
            true => (i32::from(size.0), i32::from(size.1)),
            false => (i32::from(size.0), i32::from(size.1 - 1)),
        };

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
            if row % 5 == 0 {
                stdout.flush()?;
            }
        }
        stdout.flush()?;
        Ok(Canvas {
            columns,
            rows,
            sleep_ms,
            draw_ant,
            colors: Canvas::generate_colors(number_of_states),
            stdout: RefCell::new(stdout),
        })
    }

    pub fn draw(
        &self,
        board: &HashMap<Vector2<i32>, usize>,
        ant_position: Vector2<i32>,
        ant_direction: Vector2<i32>,
    ) -> io::Result<()> {
        let cell_ant = self.complex_to_screen(ant_position);
        let cell_prev = self.complex_to_screen(ant_position - ant_direction);

        // Draw the cell where the ant was because it changed color
        self.draw_cell(cell_prev, ant_position, board)?;

        // Draw the cell with the ant if it is a different cell
        if self.draw_ant && cell_ant != cell_prev {
            self.draw_cell(cell_ant, ant_position, board)?;
        }

        Ok(())
    }

    fn draw_cell(
        &self,
        cell_location: (i32, i32),
        ant_position: Vector2<i32>,
        board: &HashMap<Vector2<i32>, usize>,
    ) -> io::Result<()> {
        if !((1 <= cell_location.0) && (cell_location.0 <= self.columns) && (1 <= cell_location.1)
            && (cell_location.1 <= self.rows))
        {
            return Ok(());
        }

        let (top, bottom) = self.screen_to_complex(cell_location.0, cell_location.1);
        let (fg, bg): ((u8, u8, u8), (u8, u8, u8)) = (
            self.square_term_color(ant_position, top, board),
            self.square_term_color(ant_position, bottom, board),
        );

        let mut out = self.stdout.borrow_mut();
        write!(
            out,
            "{}{}{}{}",
            cursor::Goto(cell_location.0 as u16, cell_location.1 as u16),
            color::Fg(color::Rgb(fg.0, fg.1, fg.2)),
            color::Bg(color::Rgb(bg.0, bg.1, bg.2)),
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
        ant_position: Vector2<i32>,
        square_position: Vector2<i32>,
        board: &HashMap<Vector2<i32>, usize>,
    ) -> (u8, u8, u8) {
        if self.draw_ant && ant_position == square_position {
            return (0, 0, 0); //The ant is black
        }
        let cell_state = board.get(&square_position).cloned().unwrap_or(0);
        self.colors[cell_state]
    }

    fn screen_to_complex(&self, column: i32, row: i32) -> (Vector2<i32>, Vector2<i32>) {
        let x = column - self.columns / 2;
        let y = self.rows - row * 2;
        let top: Vector2<i32> = Vector2::new(x, y);
        let bottom: Vector2<i32> = Vector2::new(x, y - 1);
        (top, bottom)
    }

    fn complex_to_screen(&self, cell: Vector2<i32>) -> (i32, i32) {
        let column = cell.x + self.columns / 2;
        let row = (-cell.y + self.rows) / 2;
        (column, row)
    }

    fn generate_colors(number_of_states: usize) -> Vec<(u8, u8, u8)> {
        let mut colors: Vec<(u8, u8, u8)> = Vec::new();
        colors.push((255,255,255)); //First color is always white

        let gradient = Gradient::new(vec![
            Lch::from(LinSrgb::new(0.1, 0.1, 1.0)),
            Lch::from(LinSrgb::new(0.1, 1.0, 0.1)),
            Lch::from(LinSrgb::new(1.0, 0.1, 0.1))
        ]);
        //let colors = gradient.take(number_of_states);
        for (n, color) in gradient.take(number_of_states - 1).enumerate() {
            let color = if number_of_states > 9 && n % 3 == 0 {
                color.lighten(0.2)
            } else if number_of_states > 9 && n % 3 == 1 {
                color.darken(0.2)
            } else {
                color
            };
            colors.push(Srgb::linear_to_pixel(color));
        }

        colors.clone()
    }
}

impl Drop for Canvas{
    fn drop(&mut self){
        let mut out = self.stdout.borrow_mut();
        write!(
            out,
            "{}{}{}",
            cursor::Goto(1, (self.rows + 1) as u16),
            style::Reset,
            cursor::Show
        );
        let _ = out.flush();
    }
}
