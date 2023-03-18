# dotenv-rs

forked from [dotenv](github.com/dotenv-rs/doten

A sample project using Dotenv would look like this:

```rust
extern crate dotenv_rs;

use dotenv_rs::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
    dotenv_with_prefix(&String::from("Test")).ok();

    for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
}
```

Variable substitution
----

It's possible to reuse variables in the `.env` file using `$VARIABLE` syntax.
The syntax and rules are similar to bash ones, here's the example:


```sh

VAR=one
VAR_2=two

# Non-existing values are replaced with an empty string
RESULT=$NOPE #value: '' (empty string)

# All the letters after $ symbol are treated as the variable name to replace
RESULT=$VAR #value: 'one'

# Double quotes do not affect the substitution
RESULT="$VAR" #value: 'one'

# Different syntax, same result 
RESULT=${VAR} #value: 'one'

# Curly braces are useful in cases when we need to use a variable with non-alphanumeric name
RESULT=$VAR_2 #value: 'one_2' since $ with no curly braces stops after first non-alphanumeric symbol 
RESULT=${VAR_2} #value: 'two'

# The replacement can be escaped with either single quotes or a backslash:
RESULT='$VAR' #value: '$VAR'
RESULT=\$VAR #value: '$VAR'

# Environment variables are used in the substutution and always override the local variables
RESULT=$PATH #value: the contents of the $PATH environment variable
PATH="My local variable value"
RESULT=$PATH #value: the contents of the $PATH environment variable, even though the local variable is defined
```

Dotenv will parse the file, substituting the variables the way it's described in the comments.


Using the `dotenv!` macro
------------------------------------

Add `dotenv_codegen` to your dependencies, and add the following to the top of
your crate:

```rust
#[macro_use]
extern crate dotenv_codegen;
```

Then, in your crate:

```rust
fn main() {
  println!("{}", dotenv!("MEANING_OF_LIFE"));
}
```

[dotenv]: https://github.com/bkeepers/dotenv
