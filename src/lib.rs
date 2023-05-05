use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct MalformedJsonError;

impl fmt::Display for MalformedJsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The input JSON string is malformed.")
    }
}

impl Error for MalformedJsonError {}

pub fn get_closing_string_for_partial_json(input: &str) -> Result<String, MalformedJsonError> {
    let mut stack = Vec::new();
    let mut in_string = false;
    let mut escape = false;

    for c in input.chars() {
        if in_string {
            if escape {
                escape = false;
            } else if c == '\\' {
                escape = true;
            } else if c == '"' {
                in_string = false;
            }
        } else {
            match c {
                '{' => stack.push('}'),
                '[' => stack.push(']'),
                '"' => in_string = true,
                '}' | ']' => {
                    if stack.is_empty() {
                        return Err(MalformedJsonError);
                    }
                    let last = stack.pop().unwrap();
                    if (c == '}' && last != '}') || (c == ']' && last != ']') {
                        return Err(MalformedJsonError);
                    }
                }
                _ => (),
            }
        }
    }

    if in_string {
        stack.push('"');
    }

    Ok(stack.into_iter().rev().collect())
}

pub fn complete_json(input: &str) -> Result<String, MalformedJsonError> {
    let closing = get_closing_string_for_partial_json(input)?;
    Ok(input.to_string() + &closing)
}
