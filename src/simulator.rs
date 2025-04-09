use nalgebra::{Matrix2, Vector2}; //, Vector3, Matrix3};
use std::collections::HashMap;

pub struct Simulator {
    states: Vec<Matrix2<i32>>,
    // Use a HashMap because it grows O(n) with number of steps taken by ant.
    pub board: HashMap<Vector2<i32>, usize>,
    pub ant_position: Vector2<i32>,
    pub ant_direction: Vector2<i32>,
}

impl Simulator {
    pub fn new(rotations: &str) -> Result<Self, &'static str> {
        let mut states: Vec<Matrix2<i32>> = Vec::new();
        for c in rotations.chars() {
            let rotation = match c {
                'R' => Matrix2::new(0, 1, -1, 0),
                'L' => Matrix2::new(0, -1, 1, 0),
                'U' => Matrix2::new(-1, 0, 0, -1),
                'N' => Matrix2::new(1, 0, 0, 1),
                _ => {
                    return Err("Error. Invalid rotation.");
                }
            };
            states.push(rotation);
        }

        Ok(Simulator {
            states: states.clone(),
            board: HashMap::new(),
            ant_position: Vector2::new(0, 0),   // Ant is at origin
            ant_direction: Vector2::new(-1, 0), // facing left
        })
    }

    pub fn simulate(&mut self) {
        // Get the color of the square under the ant. Default to white.
        let square_color = self.board.get(&self.ant_position).cloned().unwrap_or(0);
        // Rotate by the state of square.
        self.ant_direction = self.states[square_color] * self.ant_direction;
        //Advance the state of the square by 1, possible wrap to back to 0
        self.board
            .insert(self.ant_position, (square_color + 1) % self.states.len());
        self.ant_position += self.ant_direction; // Move the ant by its direction.
    }
}
