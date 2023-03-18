use dotenv_rs::{dotenv, dotenv_with_prefix};
use std::env;

fn main() {
    dotenv_with_prefix("Test").ok();

    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
    dotenv().ok();

    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
}
