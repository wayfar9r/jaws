//! # Cli helpers
//!
//! a variety of useful tools

// module with cli tools
pub mod cli {

    use std::fmt::Display;
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

        pub fn demand<Pf>(&self, claim: Pf) -> Result<String, InputReadError>
        where
            Pf: Fn(&str) -> bool,
        {
            let inp = self.reader.read_string()?;
            if claim(&inp) {
                return Ok(inp);
            }
            Err(InputReadError {
                msg: "wrong input".into(),
                kind: ErrorKind::InputRequirementError,
            })
        }

        /// read and check
        ///
        /// reads an input and passes it to the predicate
        /// until a predicate isn't positive
        ///
        pub fn read_until<Pf>(
            &self,
            claim: Pf,
            attempts: Option<u8>,
        ) -> Result<String, InputReadError>
        where
            Pf: Fn(&str) -> bool,
        {
            let attempts = if let Some(attempts) = attempts {
                attempts
            } else {
                3u8
            };
            for _ in 0..attempts {
                let input_str = self.read()?;
                if claim(&input_str) {
                    return Ok(input_str);
                }
            }
            Err(InputReadError {
                msg: "attempts to read input were failed".into(),
                kind: ErrorKind::AttemptsExceedError,
            })
        }

        pub fn reader(&self) -> &T
        where
            T: Reader,
        {
            &self.reader
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
    use std::io::stdin;

    use crate::cli::{ErrorKind, Input, Reader};
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
        let _input = Input::new(stdin());
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
            let res = s.parse::<u8>();
            res.is_ok()
        });
        assert!(input_result.is_err());
        assert_eq!(
            &ErrorKind::InputRequirementError,
            input_result.err().unwrap().kind()
        );
    }

    #[test]
    fn should_succeed_on_third_attempt() {
        let stdin_mock = create_stdin_mock();
        stdin_mock.add_value("what?".into());
        stdin_mock.add_value("1".into());
        stdin_mock.add_value("100".into());
        let input = Input::new(stdin_mock);
        let input_res = input.read_until(
            |s| {
                if let Ok(number) = s.parse::<u16>() {
                    return number % 100 == 0;
                }
                false
            },
            Some(3),
        );
        assert!(input_res.is_ok());
        assert!(input.reader().calls() == 3);
    }
}
