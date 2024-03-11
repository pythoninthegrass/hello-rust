use dotenvy::dotenv;
use std::env;

fn main() {
    // Load .env file
    match dotenv() {
        Ok(_) => {
            // If .env file loaded successfully, greet the user
            if let Some(name) = env::var("NAME").ok() {
                println!("Hello, {}!", name);
            } else {
                println!("Hello, world!");
            }
            // read remaining env vars other than NAME
            for item in dotenvy::dotenv_iter().unwrap() {
               let (key, value) = item.unwrap();
                if key != "NAME" {
                     println!("{}: {}", key, value);
                }
            }
        },
        Err(_) => {
            // If .env file could not be loaded or vars don't match, print to stderr
            eprintln!("Error reading env vars!");
        }
    }
}
