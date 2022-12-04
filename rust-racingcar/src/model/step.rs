use rand::Rng;
use std::fmt;

#[derive(Debug)]
#[derive(Clone)]
pub struct Step {
    pub is_moved: i32,
    pub step_number: i32
}

impl Step {
    pub fn new(step_number: i32, last_step: Option<&Step>) -> Self {
        let mut this = Self {
            is_moved: 0,
            step_number
        };
        let this_step_move_number: i32 = Self::get_this_step_move_number_by_random();
        this = Self::update_this_step_move_number(&mut this, this_step_move_number, last_step);
        this
    }

    pub fn update_this_step_move_number(&mut self, this_step_move_number: i32, last_step: Option<&Step>) -> Step {
        if let Some(last_step) = last_step {
            return Self {
                is_moved: last_step.is_moved + this_step_move_number as i32,
                step_number: self.step_number
            };
        }
        return Self {
            is_moved: this_step_move_number as i32,
            step_number: self.step_number
        }
    }

    pub fn get_this_step_move_number_by_random() -> i32 {
        let random_number = rand::thread_rng().gen_range(0, 9);
        if random_number > 4 {
            return random_number
        }
        return 0
    }

    pub fn get_step_number(&self) -> i32 {
        self.step_number
    }
}

impl fmt::Display for Step {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        for _ in 0..self.is_moved {
            print!("-");
        }
        Ok(())
    }
}
