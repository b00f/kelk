use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use std::str::FromStr;
use syn::{
    parse_macro_input, parse_quote, punctuated::Punctuated, spanned::Spanned, token::Comma, Data,
    DeriveInput, Field, Fields, GenericParam, Generics, TypeParamBound,
};

/// The attribute macro to inject the code at the beginning of entry functions
/// for the Wasm contract actor.
///
/// It can be added to the contract's instantiate, process and query functions
/// like this:
/// ```
/// use kelk::kelk_derive;
/// use kelk::context::Context;
///
/// type InstantiateMsg = ();
/// type ProcessMsg = ();
/// type QueryMsg = ();
///
/// enum Error {};
///
/// #[kelk_derive(instantiate)]
/// pub fn instantiate(ctx: Context, msg: InstantiateMsg) -> Result<(), Error> {
///    todo!();
/// }
///
/// #[kelk_derive(process)]
/// pub fn process(ctx: Context, msg: ProcessMsg) -> Result<(), Error> {
///   todo!();
/// }
///
/// #[kelk_derive(query)]
/// pub fn query(ctx: Context, msg: QueryMsg) -> Result<(), Error> {
///   todo!();
/// }
/// ```
///
/// where `InstantiateMsg`, `ProcessMsg`, and `QueryMsg` are contract defined
/// types that implement CBOR encoding.
#[proc_macro_attribute]
pub fn kelk_derive(
    _attr: proc_macro::TokenStream,
    mut item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let cloned = item.clone();
    let function = parse_macro_input!(cloned as syn::ItemFn);
    let name = function.sig.ident.to_string();

    let new_code = format!(
        r##"
        #[cfg(target_arch = "wasm32")]
        mod __wasm_export_{name} {{
            #[no_mangle]
            extern "C" fn {name}(msg_ptr: u64) -> u64 {{
                let ctx = kelk::context::OwnedContext {{
                    storage: kelk::storage::Storage::new(
                        kelk::alloc::boxed::Box::new(kelk::Kelk::new())),
                    blockchain: kelk::blockchain::Blockchain::new(
                        kelk::alloc::boxed::Box::new(kelk::Kelk::new())),
                }};
                kelk::do_{name}(&super::{name}, ctx.as_ref(), msg_ptr)
            }}
        }}
    "##,
        name = name,
    );
    let entry = proc_macro::TokenStream::from_str(&new_code).unwrap();
    item.extend(entry);
    item
}

#[proc_macro_derive(Codec)]
pub fn derive_codec(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Used in the quasi-quotation below as `#name`.
    let name = input.ident;

    // Add a bound `T: Codec` to every type parameter T.
    let generics = add_trait_bounds(input.generics, parse_quote!(Codec));
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Generate an expression to sum up the heap size of each field.
    let packed_len_body = packed_len_body(&input.data);
    let (to_bytes_body, from_bytes_body) = codec_body(&input.data);

    let expanded = quote! {
        impl #impl_generics Codec for #name #ty_generics #where_clause {

            const PACKED_LEN: usize = #packed_len_body;

            #[inline]
            fn to_bytes(&self) -> alloc::vec::Vec<u8> {
                let mut bytes = alloc::vec::Vec::with_capacity(<Self as Codec>::PACKED_LEN);
                #to_bytes_body
                bytes
            }

            #[inline]
            fn from_bytes(bytes: &[u8]) -> Self {
                Self { #from_bytes_body }
            }
        }
    };

    // Hand the output tokens back to the compiler.
    proc_macro::TokenStream::from(expanded)
}

fn packed_len_body(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    // Expands to an expression like
                    //
                    //     0 + <self.x as Codec>::PACKED_LEN + <self.y as Codec>::PACKED_LEN
                    let recurse = fields.named.iter().map(|f| {
                        let ty = &f.ty;
                        quote_spanned! {f.span()=>
                            <#ty as Codec>::PACKED_LEN
                        }
                    });

                    quote! {
                        0  #(+ #recurse)*
                    }
                }
                Fields::Unnamed(ref fields) => {
                    // Expands to an expression like
                    //
                    //     0 + <self.0 as Codec>::PACKED_LEN + <self.1 as Codec>::PACKED_LEN
                    let recurse = fields.unnamed.iter().map(|f| {
                        let ty = &f.ty;
                        quote_spanned! {f.span()=>
                            <#ty as Codec>::PACKED_LEN
                        }
                    });
                    quote! {
                        0 #(+ #recurse)*
                    }
                }
                Fields::Unit => {
                    // Unit structs cannot own more than 0 bytes of heap memory.
                    quote!(0)
                }
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

// Add a bound `T: trait_bound` to every type parameter T.
fn add_trait_bounds(mut generics: Generics, trait_bound: TypeParamBound) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(trait_bound.clone());
        }
    }
    generics
}

fn codec_body(data: &Data) -> (TokenStream, TokenStream) {
    // this also contains `bytes` variable
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => codec_fields(&fields.named),
                Fields::Unnamed(ref fields) => codec_fields(&fields.unnamed),
                Fields::Unit => {
                    // Unit structs cannot own more than 0 bytes of heap memory.
                    (quote!(0), quote!(0))
                }
            }
        }
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}

fn codec_fields(fields: &Punctuated<Field, Comma>) -> (TokenStream, TokenStream) {
    let mut beg_offset = quote! { 0 };
    let mut recurse_to_bytes = vec![];
    let mut recurse_from_bytes = vec![];

    for field in fields.iter() {
        let name = &field.ident;
        let ty = &field.ty;

        recurse_to_bytes.push(quote_spanned! {field.span()=>
            bytes.extend_from_slice(&Codec::to_bytes(&self.#name));
        });

        let struct_size = quote! { <#ty as Codec>::PACKED_LEN };
        let end_offset = quote! { #beg_offset + #struct_size };
        let bytes_slice = quote! { bytes[#beg_offset..#end_offset] };
        recurse_from_bytes.push(quote_spanned! {field.span()=>
            #name: Codec::from_bytes(& #bytes_slice),
        });

        beg_offset = quote! { #beg_offset + #struct_size };
    }

    (
        quote! {
            #(#recurse_to_bytes)*
        },
        quote! {
            #(#recurse_from_bytes)*
        },
    )
}
