
#[derive(Debug)]
pub struct RaftError(pub String);

impl RaftError {
    pub fn new(err: String) -> Self {
        Self(err)
    }
}

impl<T: std::error::Error> From<T> for RaftError {
    fn from(source: T) -> Self {
        Self(format!("RaftError({})", source.to_string()))
    }
}


pub type RaftResult<T> = std::result::Result<T, RaftError>;
