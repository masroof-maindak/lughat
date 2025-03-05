use dotenv::dotenv;
use reqwest;
use std::io;
use std::io::Write;

mod consts;
mod json;

fn gemini_contextless_message(
    client: &reqwest::blocking::Client,
    api_key: &String,
    input: String,
) -> Result<String, reqwest::Error> {
    let json_data = json::GeminiJsonRoot {
        contents: vec![json::Content {
            parts: vec![json::Part { text: input }],
        }],
    };

    let res = client
        .post(consts::TEXT_TO_TEXT_API_URL.to_string() + api_key)
        .json(&json_data)
        .send();

    let res_text = res?.text()?;

    // TODO: serialize answer to struct

    Ok(res_text)
}

fn print_help() {
    todo!();
}

fn prompt() -> String {
    print!("sahifa> ");
    io::stdout().flush().unwrap();
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
}

fn gemini_contextless_mode(
    client: &reqwest::blocking::Client,
    api_key: &String,
) -> Result<(), reqwest::Error> {
    loop {
        let input = prompt();

        if input == "\n" {
            continue;
        } else if input.is_empty() {
            println!();
            return Ok(());
        }

        if input.starts_with("/") {
            let first_word = input.split_whitespace().next().unwrap();

            match first_word {
                "/bye" => return Ok(()),
                "/help" => print_help(),
                _ => print_help(),
            }

            continue;
        }

        let resp = gemini_contextless_message(&client, &api_key, input)?;
        println!("{resp}");
    }
}

fn main() -> Result<(), reqwest::Error> {
    dotenv().ok();
    let client = reqwest::blocking::Client::new();

    // TODO: get this from config file instead
    let gemini_key = std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set");

    let _ = gemini_contextless_mode(&client, &gemini_key);

    Ok(())
}
