#[path = "input_integer.rs"] mod input_integer;

fn main() {
    println!("Rust로 구현하는 자동차 경주 게임");
    println!("자동차 대수는 몇 대인가요?");

    // 자동차 대수를 입력받는다.
    let the_number_of_cars: i32 = input_integer::input();
    println!("{:?}", the_number_of_cars);
}
