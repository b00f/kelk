use std::str::FromStr;

use proc_macro::TokenStream;
use syn::parse_macro_input;

/// The attribute macro to inject the code at the beginning of entry functions
/// for the Wasm contract actor.
///
/// It can be added to the contract's instantiate, process and query functions
/// like this:
/// ```
/// use kelk_derive::kelk_derive;
///
/// #[kelk_derive(instantiate)]
/// pub fn instantiate(ctx: Context, msg: InstantiateMsg) -> Result<(), Error> {
///    todo!()
/// }
///
/// #[kelk_derive(process)]
/// pub fn process(ctx: Context, msg: ProcessMsg) -> Result<(), Error> {
///   todo!()
/// #}
///
/// #[kelk_derive(query)]
/// pub fn query(ctx: Context, msg: QueryMsg) -> Result<(), Error> {
///   todo!()
/// }
/// ```
///
/// where `InstantiateMsg`, `ProcessMsg`, and `QueryMsg` are contract defined
/// types that implement CBOR encoding.
#[proc_macro_attribute]
pub fn kelk_derive(_attr: TokenStream, mut item: TokenStream) -> TokenStream {
    let cloned = item.clone();
    let function = parse_macro_input!(cloned as syn::ItemFn);
    let name = function.sig.ident.to_string();

    let new_code = format!(
        r##"
        #[cfg(target_arch = "wasm32")]
        mod __wasm_export_{name} {{
            #[no_mangle]
            extern "C" fn {name}(msg_ptr: u64) -> u64 {{
                kelk_env::do_{name}(&super::{name}, msg_ptr)
            }}
        }}
    "##,
        name = name,
    );
    let entry = TokenStream::from_str(&new_code).unwrap();
    item.extend(entry);
    item
}


