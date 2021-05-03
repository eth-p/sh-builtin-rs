extern crate sh_builtin_bash_bindings;
extern crate sh_builtin_bash_proc;

#[doc(hidden)]
pub mod __internal {
    pub use sh_builtin_bash_bindings::builtin as builtin_descriptor;
    pub use sh_builtin_bash_bindings::BUILTIN_ENABLED;
    pub use sh_builtin_bash_bindings::WORD_LIST;

    pub use sh_builtin_bash_bindings::EXECUTION_FAILURE;
    pub use sh_builtin_bash_bindings::EXECUTION_SUCCESS;
}

pub use sh_builtin_bash_proc::bash_builtin;
