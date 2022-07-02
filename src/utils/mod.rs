pub mod command;
pub mod error;
pub mod result;

pub use command::UserCommand;
pub use error::FibonacciError;
pub use result::{FibonacciResult, ImplementationResult};
