use combine::error::ParseError;
use combine::parser::char::{digit, spaces, string};
use combine::EasyParser;
use combine::json::JsonValue;
use combine::{between, choice, many1, optional, Parser, Stream};

#[derive(Debug, PartialEq, Clone)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(Vec<(String, JsonValue)>),
}

fn json_null<Input>() -> impl Parser<Input, Output = JsonValue>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    string("null").map(|_| JsonValue::Null)
}

fn json_bool<Input>() -> impl Parser<Input, Output = JsonValue>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    choice((
        string("true").map(|_| JsonValue::Bool(true)),
        string("false").map(|_| JsonValue::Bool(false)),
    ))
}

fn json_number<Input>() -> impl Parser<Input, Output = JsonValue>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let integer = many1::<Vec<_>, _, _>(digit());
    let fraction = (combine::token('.'), many1::<Vec<_>, _, _>(digit())).map(|(_, digits)| digits);
    let exponent = (
        choice((combine::token('e'), combine::token('E'))),
        optional(choice((combine::token('+'), combine::token('-')))),
        many1::<Vec<_>, _, _>(digit()) ,
    )
        .map(|(_, sign, digits)| (sign, digits));

    (
        optional(combine::token('-')),
        integer,
        optional(fraction),
        optional(exponent),
    )
        .map(|(sign, integer, fraction, exponent)| {
            let iss: String = integer.into_iter().collect();
            let number = iss.parse::<f64>().unwrap();
            let fraction = fraction
                .map(|frac| {
                    let fs = frac.into_iter().collect::<String>();
                    format!(".{}", fs).parse::<f64>().unwrap()
        })
                .unwrap_or(0.0);

            let (exponent_sign, exponent) = exponent
                .map(|(sign, exp)| {
                    let ess: String = exp.into_iter().collect();
                    (sign.unwrap_or('+'), ess.parse::<i32>().unwrap())
                })
                .unwrap_or(('+', 0));

            let signed_number = if sign.is_some() { -1.0 } else { 1.0 }
                * (number + fraction)
                * 10f64.powi(if exponent_sign == '-' {
                    -exponent
                } else {
                    exponent
                });

            JsonValue::Number(signed_number)
        })
}

fn json_string<Input>() -> impl Parser<Input, Output = String>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let escape_sequence = (
        combine::token('\\'),
        choice((
            combine::token('\\'),
            combine::token('"'),
            combine::token('/'),
            combine::token('b'),
            combine::token('f'),
            combine::token('n'),
            combine::token('r'),
            combine::token('t'),
        )),
    )
        .map(|(_, c)| match c {
            '\\' => '\\',
            '"' => '"',
            '/' => '/',
            'b' => '\x08',
            'f' => '\x0C',
            'n' => '\n',
            'r' => '\r',
            't' => '\t',
            _ => unreachable!(),
        });

    let string_content = many1(choice((
        combine::satisfy(|c: char| c != '\\' && c != '"'),
        escape_sequence,
    )));

    between(combine::token('"'), combine::token('"'), string_content)
}

fn json_array<Input>() -> impl Parser<Input, Output = JsonValue>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    // let parser = spaces()
    //     .with(json_value())
    //     .skip(spaces())
    //     .sep_by(combine::token(','));
    let jv = json_value();

    let parser = jv.sep_by(combine::token(','));
    between(combine::token('['), combine::token(']'), parser).map(JsonValue::Array)
}

// fn json_object<Input>() -> impl Parser<Input, Output = JsonValue>
// where
//     Input: Stream<Token = char>,
//     Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
// {
//     let key_value = (
//         json_string().skip(spaces()),
//         combine::token(':'),
//         json_value().skip(spaces()),
//     )
//         .map(|(key, _, value)| (key, value));
//     let parser = spaces()
//         .with(key_value)
//         .skip(spaces())
//         .sep_by(combine::token(','));
//     between(combine::token('{'), combine::token('}'), parser).map(JsonValue::Object)
// }

pub fn json_value<Input>() -> impl Parser<Input, Output = JsonValue>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    choice((
        json_null(),
        json_bool(),
        json_number(),
        json_string().map(JsonValue::String),
        json_array(),
        //json_object(),
    ))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_null() {
        let json_str = "null";
        let result = json_value().easy_parse(json_str).unwrap().0;
        assert_eq!(result, JsonValue::Null);
    }

    #[test]
    fn test_parse_bool() {
        let json_str = "true";
        let result = json_value().easy_parse(json_str).unwrap().0;
        assert_eq!(result, JsonValue::Bool(true));

        let json_str = "false";
        let result = json_value().easy_parse(json_str).unwrap().0;
        assert_eq!(result, JsonValue::Bool(false));
    }

    #[test]
    fn test_parse_number() {
        let json_str = "42";
        let result = json_value().easy_parse(json_str).unwrap().0;
        assert_eq!(result, JsonValue::Number(42.0));

        let json_str = "-3.14";
        let result = json_value().easy_parse(json_str).unwrap().0;
        assert_eq!(result, JsonValue::Number(-3.14));
    }

    #[test]
    fn test_parse_string() {
        let json_str = r#""hello""#;
        let result = json_value().easy_parse(json_str).unwrap().0;
        assert_eq!(result, JsonValue::String("hello".to_string()));
    }

    #[test]
    fn test_parse_array() {
        let json_str = r#"["one", 2, true]"#;
        let result = json_value().easy_parse(json_str).unwrap().0;
        assert_eq!(
            result,
            JsonValue::Array(vec![
                JsonValue::String("one".to_string()),
                JsonValue::Number(2.0),
                JsonValue::Bool(true)
            ])
        );
    }

    #[test]
    fn test_parse_object() {
        let json_str = r#"
        {
            "name": "John Doe",
            "age": 30,
            "is_student": false,
            "courses": ["math", "history"],
            "contact": {
                "email": "johndoe@example.com",
                "phone": null
            }
        }
        "#;

        let result = json_value().easy_parse(json_str).unwrap().0;
        assert_eq!(
            result,
            JsonValue::Object(vec![
                ("name".to_string(), JsonValue::String("John Doe".to_string())),
                ("age".to_string(), JsonValue::Number(30.0)),
                ("is_student".to_string(), JsonValue::Bool(false)),
                (
                    "courses".to_string(),
                    JsonValue::Array(vec![
                        JsonValue::String("math".to_string()),
                        JsonValue::String("history".to_string())
                    ])
                ),
                (
                    "contact".to_string(),
                    JsonValue::Object(vec![
                        (
                            "email".to_string(),
                            JsonValue::String("johndoe@example.com".to_string())
                        ),
                        ("phone".to_string(), JsonValue::Null)
                    ])
                )
            ])
        );
    }
}