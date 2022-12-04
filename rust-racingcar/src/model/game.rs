#[path = "Player.rs"]
pub(crate) mod player;
#[derive(Debug)]
pub struct Game {
    pub number_of_players: i32,
    pub number_of_attempts: i32,
    players: Vec<player::Player>
}

impl Game {
    pub fn new(name_of_players: Vec<String>, number_of_players: i32, number_of_attempts: i32) -> Self {
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

    pub fn get_number_of_players(&self) -> i32 {
        self.number_of_players
    }

    pub fn get_number_of_attempts(&self) -> i32 {
        self.number_of_attempts
    }

    pub fn get_players(&self) -> &Vec<player::Player> {
        &self.players
    }

    pub fn initialize_players(mut self, name_of_players: &Vec<String>) -> Self {
        for name in name_of_players {
            let new_player = player::Player::new(name.to_string(), self.number_of_attempts);
            self.players.push(new_player);
        }
        self
    }

    pub fn play_steps(self) -> Self {
        let mut this = self;
        for player in this.players.iter_mut() {
            player.play_steps();
        }
        this
    }

    pub fn print_the_dash_by_the_amount_of_is_moved_on_steps(&self) {
        for player in self.players.iter() {
            player.print_the_dash_by_the_amount_of_is_moved_on_steps();
        }
    }

    pub fn print_the_dash_by_the_amount_of_steps_on_this_step(&self, step_number: i32) {
        for player in self.players.iter() {
            player.print_the_dash_by_the_amount_of_steps_on_this_step(step_number);
        }
        println!();
    }
}
