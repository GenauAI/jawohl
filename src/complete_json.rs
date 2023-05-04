//use std::iter::Peekable;
//use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
enum RespawnReason {
    StringEscape,
    CollectionItem,
}

#[derive(Debug, PartialEq, Clone)]
struct Context {
    context_type: ContextType,
    respawn_reason: Option<RespawnReason>,
}

fn is_whitespace(c: char) -> bool {
    c.is_whitespace()
}

pub fn complete_json(json: &str) -> String {
    let mut context_stack = vec![Context {
        context_type: ContextType::TopLevel,
        respawn_reason: None,
    }];

    let mut result = String::new();
    let mut char_iter = json.chars().peekable();

    while let Some(c) = char_iter.next() {
        println!("\nstart of loop");
        dbg!(c);
        dbg!(&result);
        dbg!(context_stack.clone());
        match context_stack.last_mut().unwrap().context_type {
            ContextType::TopLevel => start_any(c, &mut context_stack),
            ContextType::String => match c {
                '"' => {
                    context_stack.pop();
                }
                '\\' => {
                    context_stack.last_mut().unwrap().respawn_reason =
                        Some(RespawnReason::StringEscape);
                    context_stack.push(Context {
                        context_type: ContextType::StringEscaped,
                        respawn_reason: None,
                    });
                }
                _ => {}
            },
            ContextType::StringEscaped => match c {
                'u' => {
                    context_stack.pop();
                    context_stack.push(Context {
                        context_type: ContextType::StringUnicode,
                        respawn_reason: None,
                    });
                }
                _ => {
                    context_stack.pop();
                    clear_respawn(
                        &mut context_stack,
                        RespawnReason::StringEscape,
                    );
                }
            },
            ContextType::StringUnicode => {
                if let Some(next) = char_iter.peek() {
                    if !next.is_digit(16) {
                        context_stack.pop();
                        clear_respawn(
                            &mut context_stack,
                            RespawnReason::StringEscape,
                        );
                    }
                }
            }
            ContextType::Number => {
                if c == '.' {
                    context_stack.pop();
                    context_stack.push(Context {
                        context_type: ContextType::NumberNeedsDigit,
                        respawn_reason: None,
                    });
                } else if c == 'e' || c == 'E' {
                    context_stack.pop();
                    context_stack.push(Context {
                        context_type: ContextType::NumberNeedsExponent,
                        respawn_reason: None,
                    });
                } else if !c.is_digit(10) {
                    context_stack.pop();
                }
            }
            ContextType::NumberNeedsDigit => {
                if c.is_digit(10) {
                    context_stack.pop();
                    context_stack.push(Context {
                        context_type: ContextType::Number,
                        respawn_reason: None,
                    });
                }
            }
            ContextType::NumberNeedsExponent => {
                if c == '+' || c == '-' {
                    context_stack.pop();
                    context_stack.push(Context {
                        context_type: ContextType::NumberNeedsDigit,
                        respawn_reason: None,
                    });
                } else if c.is_digit(10) {
                    context_stack.pop();
                    context_stack.push(Context {
                        context_type: ContextType::Number,
                        respawn_reason: None,
                    });
                }
            }
            ContextType::True => {
                let cc = char_iter.peek();
                dbg!(cc);
                if let Some(next) = cc {
                    dbg!(next);
                    if !next.is_alphabetic() {
                        context_stack.pop();
                        finish_word(&mut result, "true");
                    }
                }
                else {
                    context_stack.pop();
                    finish_word(&mut result, "true");
                }
            }
            ContextType::False => {
                if let Some(next) = char_iter.peek() {
                    if !next.is_alphabetic() {
                        context_stack.pop();
                        finish_word(&mut result, "false");
                    }
                }
            }
            ContextType::Null => {
                if let Some(next) = char_iter.peek() {
                    if !next.is_alphabetic() {
                        context_stack.pop();
                        finish_word(&mut result, "null");
                    }
                }
            }
            ContextType::ArrayNeedsValue => {
                if c == ']' {
                    context_stack.pop();
                } else if !is_whitespace(c) {
                    clear_respawn(
                        &mut context_stack,
                        RespawnReason::CollectionItem,
                    );
                    context_stack.pop();
                    context_stack.push(Context {
                        context_type: ContextType::ArrayNeedsComma,
                        respawn_reason: None,
                    });
                    start_any(c, &mut context_stack);
                }
            }
            ContextType::ArrayNeedsComma => {
                if c == ']' {
                    context_stack.pop();
                } else if c == ',' {
                    set_respawn(
                        &mut context_stack,
                        RespawnReason::CollectionItem,
                    );
                    context_stack.pop();
                    context_stack.push(Context {
                        context_type: ContextType::ArrayNeedsValue,
                        respawn_reason: None,
                    });
                }
            }
            ContextType::ObjectNeedsKey => {
                if c == '}' {
                    context_stack.pop();
                } else if c == '"' {
                    set_respawn(
                        &mut context_stack,
                        RespawnReason::CollectionItem,
                    );
                    context_stack.pop();
                    context_stack.push(Context {
                        context_type: ContextType::ObjectNeedsColon,
                        respawn_reason: None,
                    });
                    context_stack.push(Context {
                        context_type: ContextType::String,
                        respawn_reason: None,
                    });
                }
            }
            ContextType::ObjectNeedsColon => {
                if c == ':' {
                    context_stack.pop();
                    context_stack.push(Context {
                        context_type: ContextType::ObjectNeedsValue,
                        respawn_reason: None,
                    });
                }
            }
            ContextType::ObjectNeedsValue => {
                if !is_whitespace(c) {
                    clear_respawn(
                        &mut context_stack,
                        RespawnReason::CollectionItem,
                    );
                    context_stack.pop();
                    context_stack.push(Context {
                        context_type: ContextType::ObjectNeedsComma,
                        respawn_reason: None,
                    });
                    start_any(c, &mut context_stack);
                }
            }
            ContextType::ObjectNeedsComma => {
                if c == '}' {
                    context_stack.pop();
                } else if c == ',' {
                    set_respawn(
                        &mut context_stack,
                        RespawnReason::CollectionItem,
                    );
                    context_stack.pop();
                    context_stack.push(Context {
                        context_type: ContextType::ObjectNeedsKey,
                        respawn_reason: None,
                    });
                }
            }
        }
    }

    println!("Loop is done, here is what we got");
    dbg!(result.clone());
    dbg!(context_stack.clone());

    for context in context_stack.iter().rev() {
        match context.context_type {
            ContextType::String => result.push('"'),
            ContextType::NumberNeedsDigit | ContextType::NumberNeedsExponent => result.push('0'),
            ContextType::True => finish_word(&mut result, "true"),
            ContextType::False => finish_word(&mut result, "false"),
            ContextType::Null => finish_word(&mut result, "null"),
            ContextType::ArrayNeedsValue | ContextType::ArrayNeedsComma => result.push(']'),
            ContextType::ObjectNeedsKey | ContextType::ObjectNeedsColon | ContextType::ObjectNeedsValue | ContextType::ObjectNeedsComma => result.push('}'),
            _ => {}
        }
    }

    result
}

fn start_any(c: char, context_stack: &mut Vec<Context>) {
    match c {
        '"' => context_stack.push(Context {
            context_type: ContextType::String,
            respawn_reason: None,
        }),
        '-' | '0'..='9' => context_stack.push(Context {
            context_type: ContextType::Number,
            respawn_reason: None,
        }),
        't' => context_stack.push(Context {
            context_type: ContextType::True,
            respawn_reason: None,
        }),
        'f' => context_stack.push(Context {
            context_type: ContextType::False,
            respawn_reason: None,
        }),
        'n' => context_stack.push(Context {
            context_type: ContextType::Null,
            respawn_reason: None,
        }),
        '[' => context_stack.push(Context {
            context_type: ContextType::ArrayNeedsValue,
            respawn_reason: None,
        }),
        '{' => context_stack.push(Context {
            context_type: ContextType::ObjectNeedsKey,
            respawn_reason: None,
        }),
        _ => {}
    }
}

fn finish_word(result: &mut String, word: &str) {
    dbg!(word);
    dbg!(result.clone());
    result.push_str(word);
}

fn set_respawn(context_stack: &mut Vec<Context>, reason: RespawnReason) {
    if let Some(context) = context_stack.last_mut() {
        context.respawn_reason = Some(reason);
    }
}

fn clear_respawn(context_stack: &mut Vec<Context>, reason: RespawnReason) {
    let index = context_stack.iter().rposition(|c| c.respawn_reason == Some(reason));
    if let Some(index) = index {
        context_stack.remove(index);
    }
}