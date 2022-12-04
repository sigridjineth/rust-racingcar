use rand::Rng;
use std::fmt;

#[derive(Debug)]
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
        this = Self::move_if_random_number_is_more_than_four(step_number);
        this = Self::reflect_is_moved_by_last_step_is_moved(&mut this, last_step);
        this
    }

    pub fn reflect_is_moved_by_last_step_is_moved(&mut self, last_step: Option<&Step>) -> Self {
        if let Some(last_step) = last_step {
            self.is_moved += last_step.is_moved;
        }
        // hard copy self
        let hard_copied_step = Self {
            is_moved: self.is_moved,
            step_number: self.step_number
        };
        hard_copied_step
    }

    pub fn move_if_random_number_is_more_than_four(step_number: i32) -> Self {
        let mut step = Step {
            is_moved: 0,
            step_number
        };
        let random_number = rand::thread_rng().gen_range(0, 9);
        if random_number > 4 {
            step.is_moved += random_number;
        }
        step
    }

    pub fn get_step_number(&self) -> i32 {
        self.step_number
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
