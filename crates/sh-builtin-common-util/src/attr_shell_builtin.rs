use syn::{AttributeArgs, Lit, Meta, NestedMeta};

/// Attempts to parse `function = "name"` for the `bash_builtin` attribute macro.
pub fn parse_named_function(args: &AttributeArgs) -> Option<String> {
    for arg in args {
        match arg {
            NestedMeta::Meta(Meta::NameValue(v)) => {
                if v.path.is_ident("function") {
                    return match &v.lit {
                        Lit::Str(s) => Some(s.value()),
                        _ => None, // FIXME: Report an error somehow.
                    };
                }
            }
            _ => {}
        }
    }

    None
}
