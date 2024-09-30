use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("end of file")]
    EOF,
    #[error("unknown data store error")]
    Unknown,
}
