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

    pub trait Claimy<F, Ft, Fe>
    where
        F: Fn(&str) -> Result<Ft, Fe>,
        Fe: Display + Debug,
    {
        fn demand(&self, claim: F) -> Result<String, InputReadError>;

        fn demand_until(&self, claim: F, attempts: Option<u8>) -> Result<String, InputReadError>;
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

    impl<R, F, Ft, Fe> Claimy<F, Ft, Fe> for Input<R>
    where
        R: Reader,
        F: Fn(&str) -> Result<Ft, Fe>,
        Fe: Display + Debug,
    {
        fn demand(&self, claim: F) -> Result<String, InputReadError> {
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
        fn demand_until(&self, claim: F, attempts: Option<u8>) -> Result<String, InputReadError> {
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
mod tests;
