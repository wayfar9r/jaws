//! # Tools
//!
//! a variety of useful tools

// module with tools
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
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, collections::VecDeque};

    use crate::cli::{Input, Reader};

    struct StdinMock {
        values: RefCell<VecDeque<String>>,
    }

    impl StdinMock {
        fn add_value(&mut self, val: String) -> &mut StdinMock {
            self.values.borrow_mut().push_back(val);
            self
        }

        fn mock_once(&mut self) -> String {
            self.values.borrow_mut().pop_front().unwrap()
        }
    }

    impl Reader for StdinMock {
        fn read_string(&self) -> Result<String, std::io::Error> {
            Ok(self.values.borrow_mut().pop_front().unwrap())
        }
    }

    fn create_stdin_mock() -> StdinMock {
        StdinMock {
            values: RefCell::new(VecDeque::new()),
        }
    }

    #[test]
    fn test_bool_input() {
        let mut stdin_mock = create_stdin_mock();
        stdin_mock
            .add_value("true".into())
            .add_value("false".into());
        let input = Input::new(stdin_mock);
        let input_result = input.read();
        assert!(input_result.is_ok());
        assert_eq!(input_result.unwrap(), "true".to_string());
    }

    #[test]
    fn test_until_input() {
        let mut stdin_mock = create_stdin_mock();
        stdin_mock
            .add_value("what?".into())
            .add_value("0".into())
            .add_value("100".into());
        let input = Input::new(stdin_mock);
        let input_res = input.read_until(|s| {
            if let Ok(number) = s.parse::<u16>() {
                return number % 100 == 0;
            }
            false
        });
        assert!(input_res.is_ok());
    }
}
