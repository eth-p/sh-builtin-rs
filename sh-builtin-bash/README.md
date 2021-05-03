# sh-builtin-rs

A crate and proc macro for compiling dynamically-loadable Bash builtins.

**NOTE:** This crate is a work-in-progress.

## Usage

```rust
use sh_builtin_bash::bash_builtin;

/// Prints its arguments back to the standard output.
#[bash_builtin(function = "echo-rs")]
pub fn echo_rs(words: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", words.join(" "));
    Ok(())
}
```
