#[derive(Debug)]
pub struct Game {
    pub number_of_players: i32,
    pub number_of_attempts: i32
}

impl Game {
    pub fn new(number_of_players: i32, number_of_attempts: i32) -> Self {
        Self {
            number_of_players,
            number_of_attempts
        }
    }

    pub fn get_number_of_players(&self) -> i32 {
        self.number_of_players
    }

    pub fn get_number_of_attempts(&self) -> i32 {
        self.number_of_attempts
    }
}