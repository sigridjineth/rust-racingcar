use rand::Rng;
use std::fmt;

#[derive(Debug)]
pub struct Step {
    pub is_moved: i32
}

impl Step {
    pub fn new() -> Self {
        Self::move_if_random_number_is_more_than_four()
    }

    pub fn move_if_random_number_is_more_than_four() -> Self {
        let mut step = Step {
            is_moved: 0
        };
        let random_number = rand::thread_rng().gen_range(0, 9);
        if random_number > 4 {
            step.is_moved += random_number;
        }
        step
    }
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for _ in 0..self.is_moved {
            print!("-");
        }
        // write!(f, "{}", self.is_moved)
        Ok(())
    }
}