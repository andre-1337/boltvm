//! An error handler for the VM

use crate::register::Register;
use crate::types::*;

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
    ExpectedType(String, Register),
    ArrayIndexOutOfBounds(usize),
    DivisionByZero,
    LabelNotDefined(Label),
	StackUnderflow,
	StackOverflow,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let err = format!("[{} error {}]", Color::Red, Color::Reset);

        match self {
            Self::ExpectedType(expected_type, register) => {
                write!(
                    f,
                    "{err} expected type '{expected_type}' in register '{register}'"
                )
            }

            Self::ArrayIndexOutOfBounds(index) => {
                write!(f, "{err} index '{index}' is out of bounds for array")
            }

            Self::DivisionByZero => {
                write!(f, "{err} division by zero")
            }

            Self::LabelNotDefined(label) => {
                write!(f, "{err} label '{}' is not defined", label.0)
            }

			Self::StackUnderflow => {
				write!(f, "{err} stack underflow")
			}

			Self::StackOverflow => {
				write!(f, "{err} stack overflow")
			}
        }
    }
}

/// The result type used in the VM
pub type Result<T> = std::result::Result<T, Error>;
