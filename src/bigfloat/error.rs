use super::super::bigint::BigIntError;

#[derive(Debug)]
pub enum BigFloatError {
    ZeroDivisionError,
    ParseError,
}

impl From<BigIntError> for BigFloatError {
    fn from(value: BigIntError) -> Self {
        match value {
            BigIntError::ParseError => BigFloatError::ParseError,
            BigIntError::ZeroDivisionError => BigFloatError::ZeroDivisionError,
        }
    }
}