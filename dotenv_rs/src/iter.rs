use std::collections::HashMap;
use std::env;
use std::io::prelude::*;
use std::io::{BufReader, Lines};

use crate::errors::*;
use crate::parse;

pub struct Iter<R> {
    lines: Lines<BufReader<R>>,
    substitution_data: HashMap<String, Option<String>>,
}

impl<R: Read> Iter<R> {
    pub fn new(reader: R) -> Iter<R> {
        Iter {
            lines: BufReader::new(reader).lines(),
            substitution_data: HashMap::new(),
        }
    }

    pub fn load(self, prefix: &str) -> Result<()> {
        for item in self {
            let (key, value) = item?;
            if !key.starts_with(prefix) { continue; }
            if env::var(&key).is_err() {
                env::set_var(&key, value);
            }
        }

        Ok(())
    }

    pub fn get_vars_base(self, prefix: &str) -> Result<HashMap<String, Option<String>>>{
        let mut result = HashMap::new();

        for item in self {
            let (key, value) = item?;
            if !key.starts_with(prefix) { continue; }
            result.insert(key.clone(), Some(value.clone()));
        }
        println!("{:?}", result);
        Ok(result)
    }

    pub fn get_vars_with_prefix(self, prefix: &str) -> Result<HashMap<String, Option<String>>> {
        self.get_vars_base(prefix)
    }

    pub fn get_vars(self) -> Result<HashMap<String, Option<String>>> {
        self.get_vars_base(&String::from(""))
    }

}

impl<R: Read> Iterator for Iter<R> {
    type Item = Result<(String, String)>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let line = match self.lines.next() {
                Some(Ok(line)) => line,
                Some(Err(err)) => return Some(Err(Error::Io(err))),
                None => return None,
            };

            match parse::parse_line(&line, &mut self.substitution_data) {
                Ok(Some(result)) => return Some(Ok(result)),
                Ok(None) => {}
                Err(err) => return Some(Err(err)),
            }
        }
    }
}
