#[path = "player.rs"]
pub(crate) mod player;

#[derive(Debug)]
pub struct Game {
    pub number_of_players: i32,
    pub number_of_attempts: i32,
    pub(crate) players: Vec<player::Player>
}

pub trait GameFn {
    fn new(player_names: Vec<String>, the_number_of_cars: i32, the_number_of_attempts: i32) -> Self;
    fn print_the_dash_by_the_amount_of_is_moved_on_steps(&self);
    fn print_the_dash_by_the_amount_of_steps_on_this_step(&self, step_number: i32);
    fn get_players(&self) -> &Vec<player::Player>;
    fn get_number_of_players(&self) -> i32;
    fn get_number_of_attempts(&self) -> i32;
    fn initialize_players(self, player_names: &Vec<String>) -> Self;
    fn play_steps(self) -> Self;
}

use mockall::automock;
#[automock]
impl GameFn for Game {
    fn new(name_of_players: Vec<String>, number_of_players: i32, number_of_attempts: i32) -> Self {
        let new_game = Self {
            number_of_players,
            number_of_attempts,
            players: Vec::with_capacity(number_of_players as usize)
        };
        let new_game = Self::initialize_players(new_game, &name_of_players);
        let new_game = Self::play_steps(new_game);
        for step_number in 1..=new_game.number_of_attempts {
            Self::print_the_dash_by_the_amount_of_steps_on_this_step(&new_game, step_number);
        }
        new_game
    }

    // TODO: refactor to output UI
    fn print_the_dash_by_the_amount_of_is_moved_on_steps(&self) {
        for player in self.players.iter() {
            player.print_the_dash_by_the_amount_of_is_moved_on_steps();
        }
    }

    // TODO: refactor to output UI
    fn print_the_dash_by_the_amount_of_steps_on_this_step(&self, step_number: i32) {
        for player in self.players.iter() {
            player.print_the_dash_by_the_amount_of_steps_on_this_step(step_number);
        }
        println!();
    }

    fn get_players(&self) -> &Vec<player::Player> {
        &self.players
    }

    fn get_number_of_players(&self) -> i32 {
        self.number_of_players
    }

    fn get_number_of_attempts(&self) -> i32 {
        self.number_of_attempts
    }

    fn initialize_players(mut self, name_of_players: &Vec<String>) -> Self {
        for name in name_of_players {
            let new_player = player::Player::new(name.to_string(), self.number_of_attempts);
            self.players.push(new_player);
        }
        self
    }

    fn play_steps(self) -> Self {
        let mut this = self;
        for player in this.players.iter_mut() {
            player.play_steps();
        }
        this
    }
}
