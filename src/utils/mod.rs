use thiserror::Error;

pub mod input;
pub mod point;

#[derive(Error, Debug)]
pub enum AocError {
    #[error("Generic error")]
    GenericError,
}
