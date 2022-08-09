#[path = "input_integer.rs"] mod input_integer;
#[path = "game.rs"] mod game;

fn main() {
    println!("Rust로 구현하는 자동차 경주 게임");

    // 자동차 대수를 입력받는다.
    println!("자동차 대수는 몇 대인가요?");
    let the_number_of_cars: i32 = input_integer::input();
    println!("입력받은 자동차 대수: {:?}", the_number_of_cars);

    // 시도할 횟수를 입력받는다.
    println!("시도할 횟수는 몇 회인가요?");
    let the_number_of_attempts: i32 = input_integer::input();
    println!("시도할 횟수: {:?}", the_number_of_attempts);

    let initialized_game: game::Game = game::Game::new(the_number_of_cars, the_number_of_attempts);
    println!("초기화된 게임: {:?}", initialized_game);
}
