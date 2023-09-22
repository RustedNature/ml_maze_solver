mod maze;
mod player;
mod q_learn;



use q_learn::*;



fn main() {
    /* let maze: Maze = Maze::new();
    let mut player: Player = Player::new(Coordinate::new(
        maze.get_start_coordinate().get_row(),
        maze.get_start_coordinate().get_col(),
    ));
    let sleep_time = Duration::from_secs_f32(0.0);
    let mut counter: u128 = 0;
    maze.print_maze_wit_player_position(&player);
    loop {
        counter += 1;

        let next_direction = player.choose_next_action(&maze);
        player = player.move_direction(next_direction);
        sleep(sleep_time);

        maze.print_maze_wit_player_position(&player);
        if maze.is_player_in_goal(&player) {
            break;
        }
    }
    println!("YAY STRICHER HAVE FOUND THE GOAL");
    println!("Stricher needed {} steps to encounter the goal", counter) */
    let mut qq: QLearning = QLearning::new();
    qq.train_q_agent();
}
