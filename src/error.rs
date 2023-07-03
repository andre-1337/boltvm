//! An error handler for the VM

use crate::register::Register;

/// Console colors, because >RGB
#[derive(Debug, Clone)]
pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Reset,
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Self::Red => "\x1b[31m",
            Self::Green => "\x1b[32m",
            Self::Yellow => "\x1b[33m",
            Self::Blue => "\x1b[34m",
            Self::Reset => "\x1b[0m",
        };

        write!(f, "{res}")
    }
}

/// The type of error
#[derive(Debug, Clone)]
pub enum Error {
    ExpectedType(&'static str, Register),
    ArrayIndexOutOfBounds(usize),
    DivisionByZero,
    StackIsEmpty,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let err = format!("[{} error {}]", Color::Red, Color::Reset);

        match self {
            Self::ExpectedType(expected_type, register) => {
                write!(
                    f,
                    "{} expected type '{expected_type}' in register '{register}'",
                    err
                )
            }

            Self::ArrayIndexOutOfBounds(index) => {
                write!(f, "{} index '{index}' is out of bounds for array", err)
            }

            Self::DivisionByZero => {
                write!(f, "{} division by zero", err)
            }

            Self::StackIsEmpty => {
                write!(f, "{} stack is empty", err)
            }
        }
    }
}

/// The result type used in the VM
pub type Result<T> = std::result::Result<T, Error>;
