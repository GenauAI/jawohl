extern crate dotenv;
extern crate json_color;

use json_color::Colorizer;

use dotenv::dotenv;

use std::error::Error;
use std::io::{stdout, Write};

use jawohl::complete_json;

use async_openai::{
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
    Client,
};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let client = Client::new();

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .max_tokens(1024u16)
        .messages([ChatCompletionRequestMessageArgs::default()
            .content("Return an long, interesting and deeply nested JSON object. Do not include any other text. Make it compact not pretty printed.")
            .role(Role::User)
            .build()?])
        .build()?;

    let mut stream = client.chat().create_stream(request).await?;

    // For reasons not documented in OpenAI docs / OpenAPI spec,
    // the response of streaming call is different and doesn't include all the same fields.

    // From Rust docs on print: https://doc.rust-lang.org/std/macro.print.html
    //
    //  Note that stdout is frequently line-buffered by default so it may be necessary
    //  to use io::stdout().flush() to ensure the output is emitted immediately.
    //
    //  The print! macro will lock the standard output on each call.
    //  If you call print! within a hot loop, this behavior may be the bottleneck of the loop.
    //  To avoid this, lock stdout with io::stdout().lock():

    let mut lock = stdout().lock();
    let mut s: String = "".to_string();
    let colorizer = Colorizer::arbitrary();
    while let Some(result) = stream.next().await {
        match result {
            Ok(response) => {
                response.choices.iter().for_each(|chat_choice| {
                    if let Some(ref content) = chat_choice.delta.content {
                        s += content.as_str();
                        write!(lock, "new text {}\n", s).unwrap();

                        if let Ok(completed) = complete_json(&s) {
                            if let Ok(json_str) = colorizer.colorize_json_str(&completed) {
                                println!("{}", json_str);
                            }
                            else {
                                write!(lock, "cannot colorize \n").unwrap();
                            }
                        }
                        else {
                            write!(lock, "cannot complete \n").unwrap();
                        }
                    }
                });
            }
            Err(err) => {
                writeln!(lock, "error: {err}").unwrap();
            }
        }
        stdout().flush()?;
    }

    Ok(())
}
