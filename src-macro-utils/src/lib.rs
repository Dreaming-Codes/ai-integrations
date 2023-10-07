use quote::quote;
use syn::__private::TokenStream;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(SerializeError)]
pub fn serialize_error_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    let gen = quote! {
        impl serde::Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
                S: serde::Serializer {
                serializer.serialize_str(&self.to_string())
            }
        }
    };
    gen.into()
}
