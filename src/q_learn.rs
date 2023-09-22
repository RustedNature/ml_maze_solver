use std::usize;

use super::maze::*;
use super::player::*;
pub const EPISODES: usize = 200000;
pub const MAX_STEPS: usize = 100;

#[derive(Debug)]
pub struct QLearning {
    pub(crate) q_table: [[[f64; AVAILABLE_ACTIONS]; MAZE_SIZE]; MAZE_SIZE],
    pub(crate) learning_rate: f64,
    pub(crate) discount_factor: f64,
}

impl QLearning {
    pub fn new() -> Self {
        QLearning {
            q_table: [[[0.0; AVAILABLE_ACTIONS]; MAZE_SIZE]; MAZE_SIZE],
            learning_rate: 0.5,
            discount_factor: 0.9,
        }
    }

    pub fn get_action_q_value(&self, row: i32, col: i32) -> &[f64; AVAILABLE_ACTIONS] {
        if row <= -1 || col <= -1 || row >= 10 || col >= 10 {
            &[-10.0f64; AVAILABLE_ACTIONS]
        } else {
            &self.q_table[row as usize][col as usize]
        }
    }

    pub fn train_q_agent(&mut self) {
        let mut good_cnt = 0;
        let mut bad_cnt = 0;
        for _episode in 0..EPISODES {
            let maze = Maze::new();
            let mut player: Player = Player::new(Coordinate::new(
                MAZE_START_ROW as i32,
                MAZE_START_COL as i32,
            ));

            let mut steps = 0;

            while !maze.is_player_in_goal(&player) && steps < MAX_STEPS {
                steps += 1;
                let move_direction = player.q_choose_next_action(&maze, self);
                let is_legal_move = maze.is_legal_move(&move_direction, &player);

                let next_pos = player.get_next_position(&move_direction);
                let reward = &self.get_reward(next_pos.clone(), is_legal_move);

                let max_q_value = self.get_max_q_value(next_pos.clone(), is_legal_move);
                let mut current_q_value = -1.0;
                if is_legal_move {
                    current_q_value = self.q_table[next_pos.get_row() as usize]
                        [next_pos.get_col() as usize]
                        [self.get_move_direction_as_usize(move_direction)];
                }

                let updated_q_value = current_q_value
                    + self.learning_rate
                        * (reward * self.discount_factor + max_q_value - current_q_value);
                //println!("{}", updated_q_value);
                self.q_table[player.get_coordinate().get_row() as usize]
                    [player.get_coordinate().get_col() as usize]
                    [self.get_move_direction_as_usize(move_direction)] = updated_q_value;
                /* println!(
                    "Row {} Col {} on movment {}({}) shoud be now {}",
                    player.get_coordinate().get_row(),
                    player.get_coordinate().get_col(),
                    self.get_move_direction_as_usize(move_direction),
                    move_direction.get_movement_string(),
                    updated_q_value
                ); */
                if is_legal_move {
                    player = player.move_to(move_direction);
                }
                //maze.print_maze_wit_player_position(&player);
                //sleep(Duration::from_secs_f64(0.0));
            }
            if steps >= MAX_STEPS {
                // println!("Episode {} with {} steps, AGENT BAD!!!", episode, steps);
                bad_cnt += 1;
            } else {
                //println!("Episode {} with {} steps, AGENT GOOOD!!!", episode, steps);
                good_cnt += 1;
            }
        }
        println!(
            "Bads: {}\nGoods: {}\n%Goods: {}%",
            bad_cnt,
            good_cnt,
            (good_cnt as f64 / EPISODES as f64) * 100f64
        );
    }

    fn get_reward(&self, will_move_to: Coordinate, is_legal_move: bool) -> f64 {
        if will_move_to.get_row() == MAZE_GOAL_ROW as i32
            && will_move_to.get_col() == MAZE_GOAL_COL as i32
        {
            100.0
        } else if !is_legal_move {
            -2.0
        } else {
            -1.5
        }
    }

    fn get_max_q_value(&self, next_pos: Coordinate, is_legal_move: bool) -> f64 {
        let mut max_q_value = -1.0;
        if is_legal_move {
            let q_values: &[f64; 4] =
                self.get_action_q_value(next_pos.get_row(), next_pos.get_col());
            for index in 0..AVAILABLE_ACTIONS {
                let q_value: f64 = q_values[index];
                if q_value > max_q_value {
                    max_q_value = q_value;
                }
            }
        }
        max_q_value
    }

    fn get_move_direction_as_usize(&self, move_direction: MoveDirection) -> usize {
        match move_direction {
            MoveDirection::Up => 0,
            MoveDirection::Down => 1,
            MoveDirection::Left => 2,
            MoveDirection::Right => 3,
            MoveDirection::None => 0,
        }
    }
}
