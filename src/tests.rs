use crate::cli::{Claimy, ErrorKind, Input, Reader};
use mocki::{Mock, Mocki};

impl Reader for Mock<String> {
    fn read_string(&self) -> Result<String, std::io::Error> {
        Ok(self.mock_once())
    }
}

fn create_stdin_mock() -> Mock<String> {
    Mock::new()
}

#[test]
fn basic() {
    let _input = Input::default();
    // todo!("assert with Stdin");
}

#[test]
fn should_get_bool_input() {
    let stdin_mock = create_stdin_mock();
    stdin_mock.add_value("true".into());
    stdin_mock.add_value("false".into());
    let input = Input::new(stdin_mock);
    let input_result = input.read();
    assert!(input_result.is_ok());
    assert_eq!(input_result.unwrap().parse::<bool>().unwrap(), true);
}

#[test]
fn should_fail_on_wrong_input() {
    let stdin_mock = Mock::new();
    stdin_mock.add_value("no, i don't have to type what you want!".to_string());
    let input = Input::new(stdin_mock);
    let input_result = input.demand(|s| {
        return s.parse::<u8>();
    });
    assert!(input_result.is_err());
    assert_eq!(
        &ErrorKind::InputRequirementError,
        input_result.err().unwrap().kind()
    );
}

#[test]
fn should_satisfy_input_demand() {
    let stdin_mock = Mock::new();
    stdin_mock.add_value("10".to_string());
    let input = Input::new(stdin_mock);
    let input_result = input.demand(|s| {
        return s.parse::<u8>();
    });
    assert!(input_result.is_ok());
}

#[test]
fn should_succeed_on_third_attempt() {
    let stdin_mock = create_stdin_mock();
    stdin_mock.add_value("what?".into());
    stdin_mock.add_value("1".into());
    stdin_mock.add_value("100".into());
    let input = Input::new(stdin_mock);
    let input_res = input.demand_until(
        |s| match s.parse::<u16>() {
            Ok(n) if n % 100 == 0 => {
                return Ok(());
            }
            Ok(_) => Err("please type 100".to_string()),
            Err(err) => Err(err.to_string()),
        },
        Some(3),
    );
    assert!(input_res.is_ok());
    assert!(input.reader().calls() == 3);
}
