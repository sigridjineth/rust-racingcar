pub fn input() -> i32 {
    use std::io;

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("문제가 발생했습니다.");
    let x: i32 = input_line.trim().parse().expect("숫자를 입력해주세요.");
    return x;
}