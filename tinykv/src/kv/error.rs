

#[derive(Debug)]
pub struct TkvError(String);

impl<T: std::error::Error> From<T> for TkvError {
    fn from(source: T) -> Self {
        Self(format!("TkvError({})", source.to_string()))
    }
}


pub type TkvResult<T> = std::result::Result<T, TkvError>;
