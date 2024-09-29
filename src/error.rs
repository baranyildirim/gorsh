use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("end of file")]
    EOF,
    #[error("unknown data store error")]
    Unknown,
}
