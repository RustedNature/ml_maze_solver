use super::player::*;

pub const MAZE_SIZE: usize = 10;
pub const MAZE_START_ROW: usize = 0;
pub const MAZE_START_COL: usize = 0;
pub const MAZE_GOAL_ROW: usize = 7;
pub const MAZE_GOAL_COL: usize = 3;

pub struct Maze {
    fields: [[char; 10]; 10],
    start_coordinate: Coordinate,
}
impl Maze {
    pub fn new() -> Self {
        Maze {
            fields: [
                ['0', '-', '0', '0', '0', '0', '-', '0', '0', '0'],
                ['0', '-', '0', '-', '-', '0', '0', '0', '-', '0'],
                ['0', '-', '0', '-', '0', '0', '-', '0', '0', '0'],
                ['0', '-', '0', '-', '-', '-', '-', '0', '0', '0'],
                ['0', '0', '0', '-', '0', '0', '0', '0', '-', '-'],
                ['0', '0', '-', '-', '0', '0', '-', '0', '0', '0'],
                ['0', '-', '-', '0', '0', '-', '0', '0', '-', '0'],
                ['0', '-', '0', 'z', '0', '-', '0', '-', '0', '0'],
                ['0', '0', '-', '-', '-', '0', '0', '0', '0', '0'],
                ['0', '0', '0', '0', '0', '0', '0', '0', '0', '0'],
            ],
            start_coordinate: Coordinate::new(0, 0),
        }
    }

    pub fn is_legal_move(&self, action: &MoveDirection, player: &Player) -> bool {
        let mut row_move: i32 = 0;
        let mut col_move: i32 = 0;
        match action {
            MoveDirection::Up => row_move = -1,
            MoveDirection::Down => row_move = 1,
            MoveDirection::Left => col_move = -1,
            MoveDirection::Right => col_move = 1,
            MoveDirection::None => return false,
        }
        let maze_view = self.get_maze_fields();
        let player_new_row = player.get_coordinate().get_row() + row_move;
        let player_new_col = player.get_coordinate().get_col() + col_move;

        if player_new_row < 0
            || player_new_col < 0
            || player_new_row > 9
            || player_new_col > 9
            || maze_view[player_new_row as usize][player_new_col as usize] == '-'
        {
            return false;
        }
        true
    }

    pub fn get_start_coordinate(&self) -> &Coordinate {
        &self.start_coordinate
    }

    pub fn print_maze_wit_player_position(&self, player: &Player) {
        for (index_row, row) in self.fields.iter().enumerate() {
            for (index_col, cell) in row.iter().enumerate() {
                if index_col == player.get_coordinate().get_col() as usize
                    && index_row == player.get_coordinate().get_row() as usize
                {
                    print!("{} ", player.get_player_icon());
                    continue;
                }

                print!("{} ", cell);
            }
            println!();
        }
        println!();
    }

    pub fn is_player_in_goal(&self, player: &Player) -> bool {
        if player.get_coordinate().get_row() == MAZE_GOAL_ROW as i32
            && player.get_coordinate().get_col() == MAZE_GOAL_COL as i32
        {
            return true;
        }
        false
    }

    pub fn get_maze_fields(&self) -> &[[char; MAZE_SIZE]; MAZE_SIZE] {
        &self.fields
    }
}
