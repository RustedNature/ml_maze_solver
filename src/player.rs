use super::maze::Maze;
use super::q_learn::*;
use rand::Rng;
pub const AVAILABLE_ACTIONS: usize = 4;

#[derive(Debug, Clone, Copy)]
pub enum MoveDirection {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
    None,
}
impl MoveDirection {
    pub fn get_movement_string(&self) -> String {
        match self {
            MoveDirection::Up => String::from("Up"),
            MoveDirection::Down => String::from("Down"),
            MoveDirection::Left => String::from("Left"),
            MoveDirection::Right => String::from("Right"),
            MoveDirection::None => String::from("NONE"),
        }
    }
}
#[derive(Clone)]
pub struct Coordinate {
    row: i32,
    col: i32,
}
impl Coordinate {
    pub fn new(row: i32, col: i32) -> Self {
        Coordinate { row, col }
    }
    pub fn get_row(&self) -> i32 {
        self.row
    }
    pub fn get_col(&self) -> i32 {
        self.col
    }
}

#[derive(Clone)]
pub struct Player {
    coordinate: Coordinate,
    display_char: char,
}
impl Player {
    /// Creates a new [`Player`].
    pub fn new(start_coordinate: Coordinate) -> Self {
        Player {
            coordinate: start_coordinate,
            display_char: '|',
        }
    }

    pub fn get_coordinate(&self) -> &Coordinate {
        &self.coordinate
    }

    pub fn get_player_icon(&self) -> &char {
        &self.display_char
    }

    pub fn choose_next_action(&self, maze: &Maze) -> MoveDirection {
        let mut random_number_generator = rand::thread_rng();
        let mut next_action: MoveDirection;
        loop {
            let random_number: i32 = (random_number_generator.gen::<f64>() * 100f64 % 4f64) as i32;
            match random_number {
                0 => next_action = MoveDirection::Up,
                1 => next_action = MoveDirection::Down,
                2 => next_action = MoveDirection::Left,
                3 => next_action = MoveDirection::Right,
                _ => next_action = MoveDirection::None,
            }
            if maze.is_legal_move(&next_action, self) {
                break;
            }
        }

        next_action
    }

    pub fn move_direction(mut self, direction: MoveDirection) -> Player {
        let mut row_after_move: i32 = self.get_coordinate().get_row();
        let mut col_after_move: i32 = self.get_coordinate().get_col();

        match direction {
            MoveDirection::Up => row_after_move = self.coordinate.get_row() - 1,
            MoveDirection::Down => row_after_move = self.coordinate.get_row() + 1,
            MoveDirection::Left => col_after_move = self.coordinate.get_col() - 1,
            MoveDirection::Right => col_after_move = self.coordinate.get_col() + 1,
            MoveDirection::None => (),
        }
        self.coordinate = Coordinate::new(row_after_move, col_after_move);

        self
    }
    pub fn q_choose_next_action(&self, maze: &Maze, q_learn: &mut QLearning) -> MoveDirection {
        let mut random_number_generator = rand::thread_rng();
        let mut next_action: MoveDirection;
        loop {
            let random_number: i32 = (random_number_generator.gen::<f64>() * 100f64 % 4f64) as i32;
            match random_number {
                0 => next_action = MoveDirection::Up,
                1 => next_action = MoveDirection::Down,
                2 => next_action = MoveDirection::Left,
                3 => next_action = MoveDirection::Right,
                _ => next_action = MoveDirection::None,
            }
            if maze.is_legal_move(&next_action, self) {
                break;
            }
        } //
        if random_number_generator.gen_range(0..=100) < 15 {
            next_action
        } else {
            let mut max_q_value: f64 = -1.0;
            let mut best_move_direction: MoveDirection = next_action;

            for (move_direction_index, q_value) in q_learn
                .get_action_q_value(
                    self.get_coordinate().get_row(),
                    self.get_coordinate().get_col(),
                )
                .iter()
                .enumerate()
            {
                if q_value > &max_q_value {
                    max_q_value = q_value.clone();
                    match move_direction_index {
                        0 => best_move_direction = MoveDirection::Up,
                        1 => best_move_direction = MoveDirection::Down,
                        2 => best_move_direction = MoveDirection::Left,
                        3 => best_move_direction = MoveDirection::Right,
                        _ => best_move_direction = MoveDirection::None,
                    }
                }
            }
            best_move_direction
        }
    } //

    pub fn get_next_position(&self, direction: &MoveDirection) -> Coordinate {
        let mut move_value_row = 0;
        let mut move_value_col = 0;

        match direction {
            MoveDirection::Up => move_value_row = -1,
            MoveDirection::Down => move_value_row = 1,
            MoveDirection::Left => move_value_col = -1,
            MoveDirection::Right => move_value_col = 1,
            MoveDirection::None => (),
        }
        Coordinate::new(
            self.get_coordinate().get_row() + move_value_row,
            self.get_coordinate().get_col() + move_value_col,
        )
    }
}
