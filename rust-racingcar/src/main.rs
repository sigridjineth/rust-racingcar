#[path = "view/input.rs"] mod input;
#[path = "model/game.rs"] mod game;
#[path = "test/test.rs"] mod test;

fn main() {
    println!("Rust로 구현하는 자동차 경주 게임");

    // 자동차 대수를 입력받는다.
    println!("자동차 대수는 몇 대인가요?");
    let the_number_of_cars: i32 = input::input_integer();
    println!("입력받은 자동차 대수: {:?}", the_number_of_cars);

    // 시도할 횟수를 입력받는다.
    println!("시도할 횟수는 몇 회인가요?");
    let the_number_of_attempts: i32 = input::input_integer();
    println!("시도할 횟수: {:?}", the_number_of_attempts);

    println!("경주할 자동차 이름을 입력하세요(이름은 쉼표(,)를 기준으로 구분).");
    let player_names: Vec<String> = input::input_names();
    println!("입력 받은 자동차 이름, {:?}", player_names);

    let initialized_game = game::Game::new(player_names, the_number_of_cars, the_number_of_attempts);
    // println!("초기화된 게임: {:?}", initialized_game);
}
