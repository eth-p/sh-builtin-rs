use syn::{Attribute, Lit, Meta, MetaNameValue};

/// Parses a doc comment.
///
/// Returns `None` if the attribute is not a valid doc comment.
pub fn parse_doc_comment(attr: &Attribute) -> Option<String> {
    // Borrowed from structopt.
    // https://github.com/TeXitoi/structopt/blob/d16cfd264d3173fe64a883dea67e9975dc7bbb2d/structopt-derive/src/attrs.rs#L383-L395
    if !attr.path.is_ident("doc") {
        return None;
    }

    if let Ok(Meta::NameValue(MetaNameValue {
        lit: Lit::Str(text),
        ..
    })) = attr.parse_meta()
    {
        Some(text.value().trim().to_owned())
    } else {
        None
    }
}

/// Parses the doc comments from an iterator of attributes.
pub fn parse_doc_comments<'a>(attrs: impl Iterator<Item = &'a Attribute>) -> Vec<String> {
    attrs.filter_map(parse_doc_comment).collect()
}
