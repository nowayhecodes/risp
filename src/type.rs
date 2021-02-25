mod error::RispError;

pub type Result<T> = std::result::Result<T, RispError>;
pub type RispResult = Result<Box<Lval>>;