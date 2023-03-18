//! This crate provides a configuration loader in the style of the [ruby dotenv
//! gem](https://github.com/bkeepers/dotenv). This library is meant to be used
//! on development or testing environments in which setting environment
//! variables is not practical. It loads environment variables from a .env
//! file, if available, and mashes those with the actual environment variables
//! provided by the operating system.

mod errors;
mod find;
mod iter;
mod parse;

use std::env::{self, Vars};
use std::ffi::OsStr;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::Once;

pub use crate::errors::*;
use crate::find::Finder;
use crate::iter::Iter;

static START: Once = Once::new();

/// After loading the dotenv file, fetches the environment variable key from the current process.
///
/// The returned result is Ok(s) if the environment variable is present and is valid unicode. If the
/// environment variable is not present, or it is not valid unicode, then Err will be returned.
///
/// Examples:
///
/// ```no_run
///
/// use dotenv_rs;
///
/// let key = "FOO";
/// let value= dotenv_rs::var(key).unwrap();
/// ```
pub fn var<K: AsRef<OsStr>>(key: K) -> Result<String> {
    START.call_once(|| {
        dotenv().ok();
    });
    env::var(key).map_err(Error::EnvVar)
}

/// After loading the dotenv file, returns an iterator of (variable, value) pairs of strings,
/// for all the environment variables of the current process.
///
/// The returned iterator contains a snapshot of the process's environment variables at the
/// time of this invocation, modifications to environment variables afterwards will not be
/// reflected in the returned iterator.
///
/// Examples:
///
/// ```no_run
///
/// use dotenv_rs;
/// use std::io;
///
/// let result: Vec<(String, String)> = dotenv_rs::vars().collect();
/// ```
pub fn vars() -> Vars {
    START.call_once(|| {
        dotenv().ok();
    });
    env::vars()
}

/// Loads the file at the specified absolute path.
///
/// Examples
///
/// ```
/// use dotenv_rs;
/// use std::env;
/// use std::path::{Path};
///
/// let my_path = env::home_dir().and_then(|a| Some(a.join("/.env"))).unwrap();
/// dotenv_rs::from_path(my_path.as_path());
/// ```
pub fn from_path<P: AsRef<Path>>(path: P) -> Result<()> {
    let iter = Iter::new(File::open(path).map_err(Error::Io)?);
    iter.load("")
}
/// Loads the file at the specified absolute path.
/// Set the env vars with target prefix
/// Examples
///
/// ```
/// use dotenv_rs;
/// use std::env;
/// use std::path::{Path};
///
/// let my_path = env::home_dir().and_then(|a| Some(a.join("/.env"))).unwrap();
/// dotenv_rs::from_path_with_prefix(my_path.as_path(), &String::from("Test"));
/// ```
pub fn from_path_with_prefix<P: AsRef<Path>>(path: P, prefix: &str) -> Result<()> {
    let iter = Iter::new(File::open(path).map_err(Error::Io)?);
    iter.load(prefix)
}

/// Like `from_path`, but returns an iterator over variables instead of loading into environment.
///
/// Examples
///
/// ```no_run
/// use dotenv_rs;
/// use std::env;
/// use std::path::{Path};
///
/// let my_path = env::home_dir().and_then(|a| Some(a.join("/.env"))).unwrap();
/// let iter = dotenv_rs::from_path_iter(my_path.as_path()).unwrap();
///
/// for item in iter {
///   let (key, val) = item.unwrap();
///   println!("{}={}", key, val);
/// }
/// ```
pub fn from_path_iter<P: AsRef<Path>>(path: P) -> Result<Iter<File>> {
    Ok(Iter::new(File::open(path).map_err(Error::Io)?))
}

/// Loads the specified file from the environment's current directory or its parents in sequence.
///
/// # Examples
/// ```
/// use dotenv_rs;
/// dotenv_rs::from_filename("custom.env").ok();
/// ```
///
/// It is also possible to do the following, but it is equivalent to using `dotenv_rs::dotenv()`,
/// which is preferred.
///
/// ```
/// use dotenv_rs;
/// dotenv_rs::from_filename(".env").ok();
/// ```
pub fn from_filename<P: AsRef<Path>>(filename: P) -> Result<PathBuf> {
    let (path, iter) = Finder::new().filename(filename.as_ref()).find()?;
    iter.load("")?;
    Ok(path)
}
/// Loads the specified file from the environment's current directory or its parents in sequence.
/// Set the env vars with target prefix
/// 
/// # Examples
/// ```
/// use dotenv_rs;
/// dotenv_rs::from_filename_with_prefix("custom.env", &String::from("Test")).ok();
/// ```
///
/// It is also possible to do the following, but it is equivalent to using `dotenv_rs::dotenv()`,
/// which is preferred.
///
/// ```
/// use dotenv_rs;
/// dotenv_rs::from_filename_with_prefix(".env", &String::from("Test")).ok();
/// ```
pub fn from_filename_with_prefix<P: AsRef<Path>>(filename: P, prefix: &str) -> Result<PathBuf> {
    let (path, iter) = Finder::new().filename(filename.as_ref()).find()?;
    iter.load(prefix)?;
    Ok(path)
}

/// Like `from_filename`, but returns an iterator over variables instead of loading into environment.
///
/// # Examples
/// ```
/// use dotenv_rs;
/// dotenv_rs::from_filename("custom.env").ok();
/// ```
///
/// It is also possible to do the following, but it is equivalent to using `dotenv_rs::dotenv()`,
/// which is preferred.
///
/// ```no_run
/// use dotenv_rs;
/// let iter = dotenv_rs::from_filename_iter(".env").unwrap();
///
/// for item in iter {
///   let (key, val) = item.unwrap();
///   println!("{}={}", key, val);
/// }
/// ```
pub fn from_filename_iter<P: AsRef<Path>>(filename: P) -> Result<Iter<File>> {
    let (_, iter) = Finder::new().filename(filename.as_ref()).find()?;
    Ok(iter)
}

/// This is usually what you want.
/// It loads the .env file located in the environment's current directory or its parents in sequence.
///
/// # Examples
/// ```
/// use dotenv_rs;
/// dotenv_rs::dotenv().ok();
/// ```
pub fn dotenv() -> Result<PathBuf> {
    let (path, iter) = Finder::new().find()?;
    iter.load("")?;
    Ok(path)
}

/// It loads the .env file located in the environment's current directory or its parents in sequence.
/// Set the env vars with target prefix
/// 
/// # Examples
/// ```
/// use dotenv_rs;
/// dotenv_rs::dotenv_with_prefix(&String::from("Test")).ok();
/// ```
pub fn dotenv_with_prefix(prefix: &str) -> Result<PathBuf> {
    let (path, iter) = Finder::new().find()?;
    iter.load(prefix)?;
    Ok(path)
}

/// Like `dotenv`, but returns an iterator over variables instead of loading into environment.
///
/// # Examples
/// ```no_run
/// use dotenv_rs;
///
/// for item in dotenv_rs::dotenv_iter().unwrap() {
///   let (key, val) = item.unwrap();
///   println!("{}={}", key, val);
/// }
/// ```
pub fn dotenv_iter() -> Result<iter::Iter<File>> {
    let (_, iter) = Finder::new().find()?;
    Ok(iter)
}
