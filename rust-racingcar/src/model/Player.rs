#[path = "Step.rs"] mod step;
#[derive(Debug)]
#[derive(Clone)]
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
        for step_number in 0..self.steps.capacity() {
            // get last step
            let last_step = self.steps.last();
            let now_step_number = step_number + 1;
            let step = step::Step::new(now_step_number as i32, last_step);
            self.steps.push(step);
        }
    }

    pub fn get_steps(&self) -> &Vec<step::Step> {
        &self.steps
    }

    pub fn print_the_dash_by_the_amount_of_is_moved_on_steps(&self) {
        println!("{}", self.name);
        for step in self.steps.iter() {
            println!("{}", step);
        }
    }

    pub fn print_the_dash_by_the_amount_of_steps_on_this_step(&self, step_number: i32) {
        for step in self.steps.iter() {
            if step.get_step_number() == step_number {
                println!("{}: {}", self.name, step);
                break;
            }
        }
    }

    pub fn get_player_name(&self) -> String {
        self.name.to_string()
    }
}
