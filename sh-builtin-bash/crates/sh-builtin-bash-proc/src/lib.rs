use proc_macro::TokenStream;

use quote::format_ident;
use quote::quote;
use sh_builtin_common_util::attr_doc::parse_doc_comments;
use sh_builtin_common_util::attr_shell_builtin::parse_named_function;
use sh_builtin_common_util::common::ShellFunctionInfo;
use sh_builtin_common_util::tokens::CString;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, AttributeArgs, Ident, ItemFn, Result};

/// Struct for a function exported as a dynamically loaded bash builtin.
#[derive(Clone, Debug)]
struct BashFunction {
    fn_ident: Ident,
    doc: Vec<String>,

    inner: ItemFn,
}

impl ShellFunctionInfo for BashFunction {
    fn doc_comments(&self) -> &Vec<String> {
        &self.doc
    }

    fn name(&self) -> String {
        self.fn_ident.to_string()
    }
}

impl Parse for BashFunction {
    fn parse(input: ParseStream) -> Result<Self> {
        let inner = ItemFn::parse(input)?;
        Ok(BashFunction {
            fn_ident: inner.sig.ident.clone(),
            doc: parse_doc_comments(inner.attrs.iter()),
            inner,
        })
    }
}

/// Exports the current function as a Bash shell builtin function.
/// This requires the crate to be built as a cdylib.
///
/// # Example
/// ```compile
/// # use sh_builtin_bash_proc::bash_builtin;
/// #[bash_builtin(function = "hello_world")]
/// fn hello(args: &Vec<String>) -> std::result::Result<(), Box<dyn std::error::Error>> {
///     println!("Hello, world!");
///     Ok(())
/// }
/// ```
#[proc_macro_attribute]
pub fn bash_builtin(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let input = parse_macro_input!(input as BashFunction);
    let inner = &input.inner;

    // Get the desired shell function name and validate it.
    let export_symbol_name = parse_named_function(&args).unwrap_or_else(|| input.name());
    if !export_symbol_name
        .chars()
        .all(|c| c == '_' || c == ':' || c == '-' || c.is_ascii_alphanumeric())
    {
        let error = format!("'{}' is not a valid bash function name", export_symbol_name);
        return TokenStream::from(quote! {
            compile_error!(#error)
        });
    }

    // Get the identifiers for the function wrapper and the function itself.
    let func_wrapper = format_ident!("__bashexport__{}__wrapper", input.fn_ident);
    let func = &input.inner.sig.ident;

    // Get the identifier and linker symbol for the builtin's descriptor struct.
    let descriptor = format_ident!("__bashexport__{}__descriptor", input.fn_ident);
    let descriptor_symbol = format!("{}_struct", export_symbol_name);

    // Generate the contents of the descriptor fields.
    let descriptor_field_usage = CString(input.usage());
    let descriptor_field_name = CString(export_symbol_name.clone());

    // Generate the boilerplate.
    TokenStream::from(quote! {

        #[allow(non_upper_case_globals, non_snake_case)]
        #[export_name = #descriptor_symbol]
        #[link_section = ".data,shell_builtin"] // FIXME: Does this actually work on Linux?
        pub static #descriptor: ::sh_builtin_bash::__internal::builtin_descriptor = ::sh_builtin_bash::__internal::builtin_descriptor {
            name: (#descriptor_field_name) as _,
            function: Some(#func_wrapper as _),
            flags: ::sh_builtin_bash::__internal::BUILTIN_ENABLED as i32,
            long_doc: 0 as _,
            short_doc: (#descriptor_field_usage) as _,
            handle: 0 as *mut i8,
        };

        #inner

        #[allow(non_snake_case)]
        pub unsafe extern "C" fn #func_wrapper(words: *mut ::sh_builtin_bash::__internal::WORD_LIST) -> ::std::os::raw::c_int {
            let args: Vec<String> = if words.is_null() {
                vec![]
            } else {
                (&unsafe { *words }).into()
            };

            match #func(&args) {
                Ok(()) => ::sh_builtin_bash::__internal::EXECUTION_SUCCESS as _,
                Err(err) => {
                    eprintln!("{}", err);
                    ::sh_builtin_bash::__internal::EXECUTION_FAILURE as _
                }
            }
        }

    })
}
