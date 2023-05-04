#[derive(Debug, PartialEq, Clone, Copy)]
enum ContextType {
    TopLevel,
    String,
    StringEscaped,
    StringUnicode,
    Number,
    NumberNeedsDigit,
    NumberNeedsExponent,
    True,
    False,
    Null,
    ArrayNeedsValue,
    ArrayNeedsComma,
    ObjectNeedsKey,
    ObjectNeedsColon,
    ObjectNeedsValue,
    ObjectNeedsComma,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum RespawnReason {
    StringEscape,
    CollectionItem,
}

fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\r' || c == '\n' || c == '\t'
}

pub fn untruncate_json(json: &str) -> String {
    let mut context_stack: Vec<ContextType> = vec![ContextType::TopLevel];
    let mut position = 0;
    let mut respawn_position: Option<usize> = None;
    let mut respawn_stack_length: Option<usize> = None;
    let mut respawn_reason: Option<RespawnReason> = None;

    let push =
        |context_stack: &mut Vec<ContextType>, context: ContextType| context_stack.push(context);
    let replace = |context_stack: &mut Vec<ContextType>, context: ContextType| {
        let i = context_stack.len();
        context_stack[i - 1] = context;
    };
    let set_respawn = |respawn_position: &mut Option<usize>,
                       respawn_stack_length: &mut Option<usize>,
                       respawn_reason: &mut Option<RespawnReason>,
                       reason: RespawnReason,
                       position: usize,
                       context_stack: &Vec<ContextType>| {
        if respawn_position.is_none() {
            *respawn_position = Some(position);
            *respawn_stack_length = Some(context_stack.len());
            *respawn_reason = Some(reason);
        }
    };
    let clear_respawn = |respawn_reason: &mut Option<RespawnReason>, reason: RespawnReason| {
        if *respawn_reason == Some(reason) {
            *respawn_reason = None;
        }
    };
    let pop = |context_stack: &mut Vec<ContextType>| {
        context_stack.pop();
    };
    let dont_consume_character = |position: &mut usize| {
        *position -= 1;
    };

    let start_any = |char: char,
                     context_stack: &mut Vec<ContextType>,
                     push: &dyn Fn(&mut Vec<ContextType>, ContextType)| {
        if char.is_digit(10) {
            push(context_stack, ContextType::Number);
            return;
        }
        match char {
            '"' => push(context_stack, ContextType::String),
            '-' => push(context_stack, ContextType::NumberNeedsDigit),
            't' => push(context_stack, ContextType::True),
            'f' => push(context_stack, ContextType::False),
            'n' => push(context_stack, ContextType::Null),
            '[' => push(context_stack, ContextType::ArrayNeedsValue),
            '{' => push(context_stack, ContextType::ObjectNeedsKey),
            _ => (),
        }
    };

    for c in json.chars() {
        match context_stack[context_stack.len() - 1] {
            ContextType::TopLevel => {
                start_any(c, &mut context_stack, &push);
            }
            ContextType::String => match c {
                '"' => {
                    pop(&mut context_stack);
                }
                '\\' => {
                    set_respawn(
                        &mut respawn_position,
                        &mut respawn_stack_length,
                        &mut respawn_reason,
                        RespawnReason::StringEscape,
                        position,
                        &context_stack,
                    );
                    push(&mut context_stack, ContextType::StringEscaped);
                }
                _ => (),
            },
            ContextType::StringEscaped => {
                if c == 'u' {
                    push(&mut context_stack, ContextType::StringUnicode);
                } else {
                    clear_respawn(&mut respawn_reason, RespawnReason::StringEscape);
                    pop(&mut context_stack);
                }
            }
            ContextType::StringUnicode => {
                if position - json[..position].rfind('u').unwrap_or(0) == 4 {
                    clear_respawn(&mut respawn_reason, RespawnReason::StringEscape);
                    pop(&mut context_stack);
                }
            }
            ContextType::Number => {
                if c == '.' {
                    replace(&mut context_stack, ContextType::NumberNeedsDigit);
                } else if c == 'e' || c == 'E' {
                    replace(&mut context_stack, ContextType::NumberNeedsExponent);
                } else if !c.is_digit(10) {
                    dont_consume_character(&mut position);
                    pop(&mut context_stack);
                }
            }
            ContextType::NumberNeedsDigit => {
                replace(&mut context_stack, ContextType::Number);
            }
            ContextType::NumberNeedsExponent => {
                if c == '+' || c == '-' {
                    replace(&mut context_stack, ContextType::NumberNeedsDigit);
                } else {
                    replace(&mut context_stack, ContextType::Number);
                }
            }
            ContextType::True | ContextType::False | ContextType::Null => {
                if !c.is_ascii_lowercase() {
                    dont_consume_character(&mut position);
                    pop(&mut context_stack);
                }
            }
            ContextType::ArrayNeedsValue => {
                if c == ']' {
                    pop(&mut context_stack);
                } else if !is_whitespace(c) {
                    clear_respawn(&mut respawn_reason, RespawnReason::CollectionItem);
                    replace(&mut context_stack, ContextType::ArrayNeedsComma);
                    start_any(c, &mut context_stack, &push);
                }
            }
            ContextType::ArrayNeedsComma => {
                if c == ']' {
                    pop(&mut context_stack);
                } else if c == ',' {
                    set_respawn(
                        &mut respawn_position,
                        &mut respawn_stack_length,
                        &mut respawn_reason,
                        RespawnReason::CollectionItem,
                        position,
                        &context_stack,
                    );
                    replace(&mut context_stack, ContextType::ArrayNeedsValue);
                }
            }
            ContextType::ObjectNeedsKey => {
                if c == '}' {
                    pop(&mut context_stack);
                } else if c == '"' {
                    set_respawn(
                        &mut respawn_position,
                        &mut respawn_stack_length,
                        &mut respawn_reason,
                        RespawnReason::CollectionItem,
                        position,
                        &context_stack,
                    );
                    replace(&mut context_stack, ContextType::ObjectNeedsColon);
                    push(&mut context_stack, ContextType::String);
                }
            }
            ContextType::ObjectNeedsColon => {
                if c == ':' {
                    replace(&mut context_stack, ContextType::ObjectNeedsValue);
                }
            }
            ContextType::ObjectNeedsValue => {
                if !is_whitespace(c) {
                    clear_respawn(&mut respawn_reason, RespawnReason::CollectionItem);
                    replace(&mut context_stack, ContextType::ObjectNeedsComma);
                    start_any(c, &mut context_stack, &push);
                }
            }
            ContextType::ObjectNeedsComma => {
                if c == '}' {
                    pop(&mut context_stack)
                } else if c == ',' {
                    set_respawn(
                        &mut respawn_position,
                        &mut respawn_stack_length,
                        &mut respawn_reason,
                        RespawnReason::CollectionItem,
                        position,
                        &context_stack,
                    );
                    replace(&mut context_stack, ContextType::ObjectNeedsKey);
                }
            }
        }
        position += 1;
    }

    if let Some(stack_length) = respawn_stack_length {
        context_stack.truncate(stack_length);
    }
    let mut result = Vec::new();
    if let Some(pos) = respawn_position {
        result.push(json[..pos].to_owned());
    } else {
        result.push(json.to_owned());
    }

    let finish_word = |word: &str, json: &str, result: &mut Vec<String>| {
        let start_index = json.rfind(word.chars().next().unwrap()).unwrap_or(0);
        let slice = &json[start_index..];
        result.push(slice.to_owned());
    };

    for &ctx in context_stack.iter().rev() {
        match ctx {
            ContextType::String => {
                result.push("\"".to_owned());
            }
            ContextType::NumberNeedsDigit | ContextType::NumberNeedsExponent => {
                result.push("0".to_owned());
            }
            ContextType::True => {
                finish_word("true", json, &mut result);
            }
            ContextType::False => {
                finish_word("false", json, &mut result);
            }
            ContextType::Null => {
                finish_word("null", json, &mut result);
            }
            ContextType::ArrayNeedsValue | ContextType::ArrayNeedsComma => {
                result.push("]".to_owned());
            }
            ContextType::ObjectNeedsKey
            | ContextType::ObjectNeedsColon
            | ContextType::ObjectNeedsValue
            | ContextType::ObjectNeedsComma => {
                result.push("}".to_owned());
            }
            _ => (),
        }
    }
    result.join("")
}
