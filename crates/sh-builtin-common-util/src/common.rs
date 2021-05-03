/// Common properties for Rust functions exported as shell builtins.
pub trait ShellFunctionInfo {
    /// Rust documentation comments of the fn.
    fn doc_comments(&self) -> &Vec<String>;

    /// Shell function name.
    fn name(&self) -> String;

    /// Shell function usage help-text.
    fn usage(&self) -> String {
        self.doc_comments()
            .iter()
            .take_while(|&c| !c.is_empty())
            .map(|c| c.clone())
            .collect::<Vec<String>>()
            .join(" ")
    }
}
