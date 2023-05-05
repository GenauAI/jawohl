# Streaming JSON Completer

Streaming JSON Completer is a Rust library that helps you complete an incomplete JSON string by automatically adding the missing closing characters (quotes, braces, and brackets). This is particularly useful when you're dealing with streaming JSON data or working with partial JSON strings and need to ensure that the JSON is valid for further processing.

# Features

* Automatically completes incomplete JSON strings
* Handles nested objects and arrays
* Handles escaped characters inside strings
* Returns an error for malformed JSON input
* _soon_ wrappers published for Javascript and Python

# Installation

Add the following line to your Cargo.toml file under the [dependencies] section:

```toml
json-completer = "0.1.0"
```

# Usage

Here's a basic example showing how to use the JSON Completer library:

```rust
use json_completer::complete_json;

fn main() {
    let input = r#"{"key": "value", "arr": [1, 2, {"nested_key": "nested_value""#;

    match complete_json(input) {
        Ok(completion) => println!("The missing closing characters are: {}", completion),
        Err(e) => println!("Error: {}", e),
    }
}
```

For more examples and advanced usage, refer to the examples directory. In particular, the [https://github.com/GenauAI/streaming-json-completer/blob/main/examples/openai_streaming_parse/src/main.rs](OpenAI Streaming Parse) example shows how to use JSON Completer to parse a stream of JSON data from OpenAI's API (while using [https://github.com/64bit/async-openai](64bit's async-openai library )).

# Running Tests

To run the test suite for JSON Completer, simply run:

``` sh
cargo test
```

This will execute a series of tests covering various scenarios, including nested objects and arrays, escaped quotes, and malformed JSON strings.

# Contributing

We welcome contributions! If you'd like to contribute, please follow these steps:

* Fork the repository on GitHub
* Create a new branch for your feature or bugfix
* Commit your changes and push the branch to your fork
* Create a pull request against the main repository

Please ensure that your code follows Rust's best practices and includes tests for any new functionality or bugfixes.

# License

This project is licensed under the MIT License. 