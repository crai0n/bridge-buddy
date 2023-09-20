use crate::error::BBError;

pub fn single_char_from_str(input: &str) -> Result<char, BBError> {
    let mut chars = input.trim().chars();
    let char = chars.next().ok_or(BBError::ParseError(input.into(), "str too short"))?;
    match chars.next() {
        Some(_) => Err(BBError::ParseError(input.into(), "str too long")),
        None => Ok(char),
    }
}
