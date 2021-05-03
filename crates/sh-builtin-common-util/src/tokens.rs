use proc_macro2::{Literal, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};

/// A compile-time C string.
pub struct CString(pub String);

impl ToTokens for CString {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let byte_string = Literal::byte_string(format!("{}\0", self.0).as_bytes());
        tokens.append_all(quote! {
            (#byte_string as *const u8)
        });
    }
}
