use std::io;
use mockall::automock;

#[automock]
trait Stdin {
    fn mock_input_integer(&mut self, buf: &mut String) -> io::Result<usize>;
    fn mock_input_names(&mut self, buf: &mut String) -> io::Result<usize>;
}

fn mock_input_integer(buf: &mut String) -> io::Result<usize> {
    buf.push_str("3");
    Ok(1)
}

fn mock_input_names(buf: &mut String) -> io::Result<usize> {
    buf.push_str("pobi,crong,honux");
    Ok(1)
}

#[test]
fn test_mock_input_integer() {
    let mut mock = MockStdin::new();
    mock.expect_mock_input_integer()
        .times(1)
        .returning(mock_input_integer);
    let mut buf = String::new();
    mock.mock_input_integer(&mut buf).unwrap();
    assert_eq!(buf, "3");
}

#[test]
fn test_mock_input_names() {
    let mut mock = MockStdin::new();
    mock.expect_mock_input_names()
        .times(1)
        .returning(mock_input_names);
    let mut buf = String::new();
    mock.mock_input_names(&mut buf).unwrap();
    assert_eq!(buf, "pobi,crong,honux");
}
