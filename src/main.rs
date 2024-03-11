use dotenvy::dotenv;
use std::env;

// TODO: only return specific env vars

fn main() {
    // Store environment variables in a vector
    let env_vars: Vec<_> = env::vars().collect();

    // dotenvy iterator returns result, unwrap to get the value, map
    // extracts the value from the result, and collect key/value
    // pairs into a vector
    let dotenv_vars: Vec<_> = dotenvy::dotenv_iter().unwrap().map(|x| x.unwrap()).collect();

    // Print env vars to stdout
    match dotenv() {
        Ok(_) => {
            // If env var is found, greet the user
            let name = env_vars
                .iter()
                .chain(dotenv_vars.iter())
                .find(|(key, _)| key == "NAME")
                .map(|(_, value)| value.to_string())
                .unwrap_or("world".to_string());

            println!("Hello, {}!\n", name);

            // read env vars from env::vars()
            println!("env vars:");
            for (key, value) in &env_vars {
                if key != "NAME" {
                    println!("{}: {}", key, value);
                }
            }
            println!();

            // read env vars from dotenvy::dotenv_iter()
            println!("dotenv vars:");
            for (key, value) in &dotenv_vars {
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
