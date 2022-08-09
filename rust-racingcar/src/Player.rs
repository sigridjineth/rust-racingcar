#[path = "Step.rs"] mod step;
#[derive(Debug)]
pub struct Player {
    name: String,
    steps: Vec<step::Step>
}

impl Player {
    pub fn new(name: String, number_of_attempts: i32) -> Self {
        Self {
            name,
            steps: Vec::with_capacity(number_of_attempts as usize)
        }
    }
}