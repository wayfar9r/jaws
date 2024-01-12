//! # Cli helpers
//!
//! a variety of useful tools

// module with cli tools
pub mod cli {

    use std::fmt::{Debug, Display};
    use std::io::{self};
    use std::io::{stdin, Stdin};

    pub trait Reader {
        fn read_string(&self) -> Result<String, io::Error>;
    }

    #[derive(Debug, PartialEq)]
    pub enum ErrorKind {
        IoError,
        AttemptsExceedError,
        InputRequirementError,
    }

    #[derive(Debug)]
    pub struct InputReadError {
        msg: String,
        kind: ErrorKind,
    }

    impl InputReadError {
        pub fn kind(&self) -> &ErrorKind {
            &self.kind
        }
    }

    impl Display for InputReadError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.msg)
        }
    }

    impl From<io::Error> for InputReadError {
        fn from(value: io::Error) -> Self {
            InputReadError {
                msg: value.to_string(),
                kind: ErrorKind::IoError,
            }
        }
    }

    pub trait Claimy {
        fn demand<F, Ft, Fe>(&self, claim: F) -> Result<String, InputReadError>
        where
            Fe: Display + Debug,
            F: Fn(&str) -> Result<Ft, Fe>;

        fn demand_until<F, Ft, Fe>(
            &self,
            claim: F,
            attempts: Option<u8>,
        ) -> Result<String, InputReadError>
        where
            Fe: Display + Debug,
            F: Fn(&str) -> Result<Ft, Fe>;
    }

    pub struct Input<T>
    where
        T: Reader,
    {
        reader: T,
    }

    impl Reader for Stdin {
        fn read_string(&self) -> Result<String, io::Error> {
            let mut buf = String::new();
            self.read_line(&mut buf)?;
            Ok(buf)
        }
    }

    impl<T> Input<T>
    where
        T: Reader,
    {
        pub fn new(reader: T) -> Input<T> {
            Input { reader }
        }

        pub fn read(&self) -> Result<String, InputReadError> {
            let inp = self.reader.read_string()?;
            Ok(inp)
        }

        pub fn reader(&self) -> &T
        where
            T: Reader,
        {
            &self.reader
        }
    }

    impl<R> Claimy for Input<R>
    where
        R: Reader,
    {
        fn demand<F, Ft, Fe>(&self, claim: F) -> Result<String, InputReadError>
        where
            Fe: Debug + Display,
            F: Fn(&str) -> Result<Ft, Fe>,
        {
            let inp = self.reader.read_string()?;
            match claim(&inp) {
                Ok(_) => Ok(inp),
                Err(err) => Err(InputReadError {
                    msg: format!("wrong input. {}", err),
                    kind: ErrorKind::InputRequirementError,
                }),
            }
        }

        /// read and check
        ///
        /// reads an input and passes it to the predicate
        /// until a predicate isn't positive
        ///
        fn demand_until<F, Ft, Fe>(
            &self,
            claim: F,
            attempts: Option<u8>,
        ) -> Result<String, InputReadError>
        where
            Fe: Display + Debug,
            F: Fn(&str) -> Result<Ft, Fe>,
        {
            let attempts = if let Some(attempts) = attempts {
                attempts
            } else {
                3u8
            };
            let mut last_error_msg = "".to_string();
            for attempt in 0..attempts {
                let input_str = self.read()?;
                let claim_res = claim(&input_str);
                if claim_res.is_ok() {
                    return Ok(input_str);
                } else if attempt == attempts - 1 {
                    last_error_msg = claim_res.err().unwrap().to_string();
                }
            }
            Err(InputReadError {
                msg: format!("attempts to read input were failed. {}", last_error_msg),
                kind: ErrorKind::AttemptsExceedError,
            })
        }
    }

    impl Default for Input<Stdin> {
        fn default() -> Self {
            Input { reader: stdin() }
        }
    }
}

#[cfg(test)]
mod tests {
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
}
