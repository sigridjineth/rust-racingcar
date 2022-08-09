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
    pub fn play_steps(&mut self) {
        for _ in 0..self.steps.capacity() {
            let step = step::Step::new();
            self.steps.push(step);
        }
    }
    pub fn print_the_dash_by_the_amount_of_is_moved_on_steps(&self) {
        println!("{}", self.name);
        for step in self.steps.iter() {
            println!("{}", step);
        }
    }
}