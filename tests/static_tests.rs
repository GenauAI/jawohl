
#[cfg(test)]
mod static_tests {
    use streaming_json_completer::complete_json::complete_json;
    use streaming_json_completer::complete_json2::untruncate_json;
    use serde_json::json;

    fn expect_unchanged(json: &str) {
        assert_eq!(untruncate_json(json), "".to_string());
    }

    #[test]
    fn unmodified_valid_string() {
        expect_unchanged("\"Hello\"");
    }

    #[test]
    fn unmodified_valid_string_with_bracket_characters() {
        expect_unchanged("}\"{][\"");
    }

    #[test]
    fn unmodified_valid_string_with_escaped_quotes() {
        expect_unchanged("\"\\\"Dr.\\\" Leo Spaceman\"");
    }

    #[test]
    fn unmodified_valid_string_with_unicode_escapes() {
        expect_unchanged("ab\\u0065cd");
    }

    #[test]
    fn unmodified_valid_number() {
        expect_unchanged("20");
    }

    #[test]
    fn unmodified_valid_boolean() {
        expect_unchanged("true");
        expect_unchanged("false");
    }

    #[test]
    fn unmodified_valid_null() {
        expect_unchanged("null");
    }

    #[test]
    fn unmodified_valid_array() {
        expect_unchanged("[]");
        expect_unchanged("[\"a\", \"b\", \"c\"]");
        expect_unchanged("[ 1, 2, 3 ]");
    }

    #[test]
    fn unmodified_valid_object() {
        expect_unchanged("{}");
        expect_unchanged("{\"foo\": \"bar\"}");
        expect_unchanged("{ \"foo\": 2 }");
    }

    #[test]
    fn unmodified_compound_object() {
        let value = json!({
            "s": "Hello",
            "num": 10,
            "b": true,
            "nul": "null",
            "o": { "s": "Hello2", "num": 11 },
            "a": ["Hello", 10, { "s": "Hello3" }],
        });

        let json_string = value.to_string();
        dbg!(json_string.clone());
        expect_unchanged(&json_string);
    }


    #[test]
    fn should_produce_valid_json_whatever_the_truncation_occurs() {
        let json = r#"{
        "ab\nc\u0065d": ["ab\nc\u0065d", true, false, null, -12.3e-4],
        "": { "12": "ab\nc\u0065d"}
    }  "#;

        for i in 1..json.len() {
            let partial_json = &json[..i];
            let fixed_json = untruncate_json(partial_json);

            assert!(serde_json::from_str::<serde_json::Value>(&fixed_json).is_ok(),
                    "Failed to produce valid JSON. \n\nInput:\n\n{}\n\nOutput (invalid JSON):\n\n{}\n\n",
                    partial_json, fixed_json);
        }
    }
}
