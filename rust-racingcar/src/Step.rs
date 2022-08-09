use rand::Rng;

#[derive(Debug)]
pub struct Step {
    is_moved: bool
}

impl Step {
    pub fn new() -> Self {
        Self::move_if_random_number_is_more_than_four()
    }

    pub fn move_if_random_number_is_more_than_four() -> Self {
        let is_moved = rand::thread_rng().gen_range(0, 9) > 4;
        Self { is_moved }
    }
}