//! # Cli helpers
//!
//! a variety of useful tools

// module with cli tools
pub mod cli {

    use std::fmt::Display;
    use std::io::Stdin;
    use std::io::{self};

    pub trait Reader {
        fn read_string(&self) -> Result<String, io::Error>;
    }

    #[derive(Debug)]
    pub struct InputReadError(String);

    impl Display for InputReadError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl From<io::Error> for InputReadError {
        fn from(value: io::Error) -> Self {
            InputReadError(value.to_string())
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

        /// read and check
        /// 
        /// reads an input and passes it to the predicate
        /// until a predicate isn't positive
        ///
        pub fn read_until<Pf>(&self, p: Pf) -> Result<String, InputReadError>
        where
            Pf: Fn(&str) -> bool,
        {
            loop {
                let input_str = self.read()?;
                if p(&input_str) {
                    return Ok(input_str);
                }
            }
        }

        pub fn reader(&self) -> &T
        where T: Reader {
            &self.reader
        }
    }
}

#[cfg(test)]
mod tests {
    use mocki::{Mock, Mocki};
    use crate::cli::{Input, Reader};

    impl Reader for Mock<String> {
        fn read_string(&self) -> Result<String, std::io::Error> {
            Ok(self.mock_once())
        }
    }

    fn create_stdin_mock() -> Mock<String> {
        Mock::new()
    }

    #[test]
    fn test_bool_input() {
        let stdin_mock = create_stdin_mock();
        stdin_mock
            .add_value("true".into());
        stdin_mock.add_value("false".into());
        let input = Input::new(stdin_mock);
        let input_result = input.read();
        assert!(input_result.is_ok());
        assert_eq!(input_result.unwrap(), "true".to_string());
    }

    #[test]
    fn test_until_input() {
        let stdin_mock = create_stdin_mock();
        stdin_mock.add_value("what?".into());
        stdin_mock.add_value("1".into());
        stdin_mock.add_value("100".into());
        let input = Input::new(stdin_mock);
        let input_res = input.read_until(|s| {
            if let Ok(number) = s.parse::<u16>() {
                return number % 100 == 0;
            }
            false
        });
        assert!(input_res.is_ok());
        assert!(input.reader().calls() == 3);
    }
}
