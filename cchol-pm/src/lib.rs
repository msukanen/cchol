use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(HasRollRange)]
pub fn derive_has_roll_range(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    TokenStream::from(quote! {
        impl UseRollRange for #name {
            fn roll_range(&self) -> &RollRange {
                &self._cr_range
            }
        }
    })
}

#[proc_macro_derive(Gendered)]
pub fn derive_has_gender(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    TokenStream::from(quote! {
        impl HasGender for #name {
            fn gender(&self) -> Gender {
                self.gender
            }
        }
    })
}

#[proc_macro_derive(HasName)]
pub fn derive_has_name(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    TokenStream::from(quote! {
        impl IsNamed for #name {
            fn name(&self) -> &str {
                &self.name
            }
        }
    })
}

#[proc_macro_derive(HasSolMod)]
pub fn derive_has_solmod(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    TokenStream::from(quote! {
        impl SolMod for #name {
            fn solmod(&self) -> i32 {
                self.solmod
            }
        }
    })
}

#[proc_macro_derive(HasCuMod)]
pub fn derive_has_cumod(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    TokenStream::from(quote! {
        impl CuMod for #name {
            fn cumod(&self) -> i32 {
                self.cumod
            }
        }
    })
}

#[proc_macro_derive(HasTiMod)]
pub fn derive_has_timod(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    TokenStream::from(quote! {
        impl TiMod for #name {
            fn timod(&self) -> i32 {
                self.timod
            }
        }
    })
}