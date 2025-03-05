use anyhow::anyhow;
use reqwest;
use std::fs::create_dir_all;
use std::io;
use std::io::Write;
use std::path::PathBuf;

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

pub fn get_cache_dir_path() -> anyhow::Result<PathBuf> {
    if let Ok(cache_home) = std::env::var("XDG_CACHE_HOME") {
        return Ok(PathBuf::from(cache_home).join("lughat"));
    }

    let mut home: PathBuf;

    match dirs_next::home_dir() {
        Some(path) => home = path,
        None => return Err(anyhow!("Failed to get home directory")),
    }

    home.push(consts::DEFAULT_CACHE_DIR);
    Ok(home.join("lughat"))
}

fn get_api_key() -> anyhow::Result<String> {
    let mut cache_home = get_cache_dir_path()?;

    create_dir_all(&cache_home)?;

    cache_home.push("gemini-api-key");

    let key = match std::fs::read_to_string(&cache_home) {
        Ok(k) => k,
        Err(e) => return Err(anyhow!("Failed to read file {:?}: {}", cache_home, e)),
    };

    Ok(key)
}

fn main() -> anyhow::Result<()> {
    let client = reqwest::blocking::Client::new();
    let gemini_key = get_api_key()?;

    println!("{}", consts::ASCII_ART);
    println!();

    let _ = gemini_contextless_mode(&client, &gemini_key);

    Ok(())
}
