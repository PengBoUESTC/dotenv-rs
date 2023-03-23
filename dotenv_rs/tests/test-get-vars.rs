mod common;

use std::{env, collections::HashMap};

use dotenv_rs::*;

use crate::common::*;

#[test]
fn test_get_vars() {
    let dir = make_test_dotenv().unwrap();

    let mut path = env::current_dir().unwrap();
    path.push(".env");

    
    if let Ok(vars) = get_vars_with_prefix(&path, &String::from("TEST"))  {
      let mut temp = HashMap::new();
      temp.insert(String::from("TESTKEY"), Some(String::from("test_val")));
      assert_eq!(vars, temp);
    } else {
        panic!("get vars error")
    };

    if let Ok(vars) = get_vars(&path)  {
      let mut temp = HashMap::new();
      temp.insert(String::from("TESTKEY"), Some(String::from("test_val")));
      temp.insert(String::from("TestKEY"), Some(String::from("test_val_prefix")));
      assert_eq!(vars, temp);
    } else {
        panic!("get vars error")
    };

    env::set_current_dir(dir.path().parent().unwrap()).unwrap();
    dir.close().unwrap();
}
