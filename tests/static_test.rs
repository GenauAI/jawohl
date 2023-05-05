#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use streaming_json_completer::{json_value, Value::*};

    use combine::EasyParser;

    #[test]
    fn json_test() {
        let input = r#"{
    "array": [1, ""],
    "object": {},
    "number": 3.15,
    "small_number": 0.59,
    "int": -100,
    "exp": -1e2,
    "exp_neg": 23e-2,
    "true": true,
    "false"  : false,
    "null" : null
}"#;
        let result = json_value().easy_parse(input);
        let expected = Object(
            vec![
                ("array", Array(vec![Number(1.0), String("".to_string())])),
                ("object", Object(HashMap::new())),
                ("number", Number(3.15)),
                ("small_number", Number(0.59)),
                ("int", Number(-100.)),
                ("exp", Number(-1e2)),
                ("exp_neg", Number(23E-2)),
                ("true", Bool(true)),
                ("false", Bool(false)),
                ("null", Null),
            ]
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect(),
        );
        match result {
            Ok(result) => assert_eq!(result, (expected, "")),
            Err(e) => {
                println!("{}", e);
                panic!();
            }
        }
    }
}
