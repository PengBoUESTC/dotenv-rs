mod common;

use dotenv_rs::*;
use std::{env};

use crate::common::*;

#[test]
fn test_dotenv_prefix() {
    let dir = make_test_dotenv().unwrap();

    dotenv_with_prefix(&String::from("Test")).ok();
    assert_eq!(env::var("TESTKEY").is_err(), true);
    assert_eq!(env::var("TestKEY").unwrap(), "test_val_prefix");

    env::set_current_dir(dir.path().parent().unwrap()).unwrap();
    dir.close().unwrap();
}
