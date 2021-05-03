extern crate sh_builtin_bash;

pub use sh_builtin_bash::bash_builtin;

/// Prints its arguments back to standard outputs.
#[bash_builtin(function = "echo-rs")]
pub fn echo_rs(_words: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", _words.join(" "));
    Ok(())
}
