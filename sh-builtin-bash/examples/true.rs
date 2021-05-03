extern crate sh_builtin_bash;

pub use sh_builtin_bash::bash_builtin;

/// Always returns a success result.
#[bash_builtin(function = "true-rs")]
pub fn true_rs(_words: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
