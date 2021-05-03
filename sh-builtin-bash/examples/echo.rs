extern crate sh_builtin_bash;

pub use sh_builtin_bash::bash_builtin;

/// Prints its arguments back to the standard output.
#[bash_builtin(function = "echo-rs")]
pub fn echo_rs(words: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", words.join(" "));
    Ok(())
}
