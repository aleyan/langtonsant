use nalgebra::{Vector2, Matrix2};//, Vector3, Matrix3};
use std::collections::HashMap;

use canvas;

pub struct Simulator {
    states: Vec<Matrix2<i32>>,
}

impl Simulator {
    pub fn new(rotations: &str) -> Result<Self, &'static str>{
        let mut states: Vec<Matrix2<i32>> = Vec::new();
        for c in rotations.chars() {
            let rotation = match c {
                'R' => Matrix2::new(0, 1,-1, 0),
                'L' => Matrix2::new(0, -1,1, 0),
                'U' => Matrix2::new(-1, 0,0, -1),
                'N' => Matrix2::new(1, 0,0, 1),
                _ => {
                    return Err("Error. Invalid rotation.");
                }
            };
            states.push(rotation);
        }
        let states = states.clone();

        Ok(Simulator{states: states.clone()})
    }

    pub fn simulate(&self, canvas: &canvas::Canvas, steps: u64){
            // Use a HashMap because it grows O(n) with number of steps taken by ant.
        let mut board: HashMap<Vector2<i32>, usize> = HashMap::new();
        let mut ant_position: Vector2<i32> = Vector2::new(0, 0); // Ant is at origin
        let mut ant_direction: Vector2<i32> = Vector2::new(-1, 0); // facing left.

        for _ in 0..steps {
            // Get the color of the square under the ant. Default to white.
            let square_color = board.get(&ant_position).cloned().unwrap_or(0);
            // Rotate by the state of square.
            ant_direction = self.states[square_color] * ant_direction;
            //Advance the state of the square by 1, possible wrap to back to 0
            board.insert(ant_position, (square_color + 1) % self.states.len());
            ant_position += ant_direction; // Move the ant by its direction.

            match canvas.draw(&board, ant_position, ant_direction) {
                Ok(_) => {}
                Err(_) => {
                    continue;
                }
            }
        }
    }
}
