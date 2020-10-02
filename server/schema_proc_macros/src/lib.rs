extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn create_options(input: TokenStream) -> TokenStream {
    let mut output_buffer = TokenStream::new();
}
