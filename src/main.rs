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
        },
        Err(_) => {
            // If .env file could not be loaded or vars don't match, print "Hello, world!"
            println!("Hello, world!");
        }
    }
}
