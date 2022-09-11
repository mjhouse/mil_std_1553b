
pub type Result<T> = core::result::Result<T,Error>;

#[derive(Debug)]
pub enum Error {

    OutOfBounds,

}

impl Error {

    pub fn to_string(&self) -> &str {
        match self {
            Error::OutOfBounds => "Value is out of bounds"
        }
    }

}