use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn kelk_derive(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    item
}
