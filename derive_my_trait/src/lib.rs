use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(MyTrait)]
pub fn derive_my_trait(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let DeriveInput { ident, .. } = input;

    let token_stream = quote! {
        fn as_i32(&self) -> i32{
            self.number
        }
    };

    let output = quote! {
        impl MyTrait for #ident {
            #token_stream
        }
    };
    output.into()
}
