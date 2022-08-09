pub fn input_integer() -> i32 {
    use std::io;

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("문제가 발생했습니다.");
    let x: i32 = input_line.trim().parse().expect("숫자를 입력해주세요.");
    return x;
}

pub fn input_names() -> Vec<String> {
    use std::io;
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("문제가 발생했습니다.");
    let names: String = input_line.trim().parse().expect("이름을 입력해주세요.");
    let names = names.split(",").map(|x| x.to_string()).collect();
    names
}