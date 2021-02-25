#[derive(Debug)]
pub enum RispError {
    DivideByZero,
    EmptyList,
    FunctionFormat,
    NoChildren,
    NotANumber,
    NumArgs(usize, usize),
    ParseError(String),
    ReadlineError(String),
    WrongType(String, String,
    UnknownFn(String)
}

impl<T> From <pest::error::Error<T>> for RispError
where
    T: Debug + Ord + Copy + Hash,
{
    fn from(error: pest::error::Error<T>) -> Self {
        RispError::ParseError(format!("{}", error))
    }
}

impl From<std::io::Error> for RispError {
    fn from(error: std::io::Error) -> Self {
        RispError::ParseError(error.to_string())
    }
}