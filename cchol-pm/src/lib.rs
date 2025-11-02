use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(HasRollRange)]
pub fn derive_has_roll_range(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    TokenStream::from(quote! {
        impl HasRollRange for #name {
            fn roll_range(&self) -> &RollRange {
                &self._cr_range
            }
        }
    })
}