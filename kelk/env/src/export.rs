//! Kelk public API
//!
//! `do_deploy`, `do_process` and `do_query`
//! should be wrapped with a extern "C" entry point including
//! the contract-specific function pointer.
//! This is done via the `#[entry_point]` macro attribute.

use crate::context::{Context, OwnedContext};
use crate::import::ContextExt;
use crate::memory::Pointer;
use minicbor::{Decode, Encode};

/// allocate reserves the given number of bytes in wasm memory and returns a pointer
/// to a Pointer defining this data. This space is managed by the calling process
/// and should be accompanied by a corresponding deallocate
#[no_mangle]
extern "C" fn allocate(size: u32) -> u64 {
    Pointer::allocate(size).as_u64()
}

/// deallocate expects a pointer to a Pointer created with allocate.
/// It will free both the Pointer and the memory referenced by the Pointer.
#[no_mangle]
extern "C" fn deallocate(ptr_u64: u64) {
    Pointer::from_u64(ptr_u64).deallocate();
}

/// do_instantiate should be wrapped in an external "C" export,
/// containing a contract-specific function as arg.
pub fn do_instantiate<E: Encode>(_instantiate_fn: &dyn Fn(Context) -> Result<(), E>) -> u32 {
    //let ctx = make_context();
    //instantiate_fn(ctx.as_ref());
    0
}

/// do_process_msg should be wrapped in an external "C" export,
/// containing a contract-specific function as arg.
pub fn do_process_msg<'a, D: Decode<'a>, E: Encode>(
    process_msg_fn: &dyn Fn(Context, D) -> Result<(), E>,
    msg_ptr: u64,
) -> u64 {
    let ptr = Pointer::from_u64(msg_ptr);
    let buf = unsafe { ptr.to_slice() };
    let msg = minicbor::decode(buf).expect("Decoding failed");
    let ctx = make_context();
    let res = process_msg_fn(ctx.as_ref(), msg);

    result_to_ptr(res)
}

/// do_query should be wrapped in an external "C" export,
/// containing a contract-specific function as arg.
pub fn do_query<'a, D: Decode<'a>, R: Encode, E: Encode>(
    query_fn: &dyn Fn(Context, D) -> Result<R, E>,
    msg_ptr: u64,
) -> u64 {
    let ptr = Pointer::from_u64(msg_ptr);
    let buf = unsafe { ptr.to_slice() };
    let msg = minicbor::decode(buf).expect("Decoding failed");
    let ctx = make_context();
    let res = query_fn(ctx.as_ref(), msg);

    result_to_ptr(res)
}

fn result_to_ptr<E: Encode>(res: E) -> u64 {
    let mut vec = kelk_lib::alloc::vec::Vec::new();
    minicbor::encode(res, &mut vec).expect("Encoding failed");

    Pointer::release_buffer(vec).as_u64()
}

/// Make context instance
pub(crate) fn make_context() -> OwnedContext<ContextExt> {
    OwnedContext {
        api: ContextExt::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kelk_lib::alloc::vec;
    use wasm_bindgen_test::*;

    // Uncomment this test if should_panic supported by wasm_bindgen_test.
    // https://github.com/rustwasm/wasm-bindgen/issues/2286
    //
    // #[wasm_bindgen_test]
    // #[should_panic]
    // fn test_allocation() {
    //     let ptr = allocate(1);
    //     deallocate(ptr);

    //     // Should panic here, because the pointer is freed before
    //     deallocate(ptr);
    // }

    #[wasm_bindgen_test]
    fn test_process_msg() {
        let msg_data = vec![0x00]; // http://cbor.me/?bytes=8100
        let msg_ptr = Pointer::release_buffer(msg_data);

        let res_ptr = do_process_msg(
            &|_: Context, _: i32| -> Result<(), i32> { Ok(()) },
            msg_ptr.as_u64(),
        );

        let res_data = unsafe { Pointer::from_u64(res_ptr).to_slice() };
        assert_eq!(res_data, vec![0x82, 0x00, 0x80]); // Result::Ok(()) -> http://cbor.me/?bytes=820080
    }

    #[wasm_bindgen_test]
    fn test_process_msg_error() {
        let msg_data = vec![0x00]; // http://cbor.me/?bytes=8100
        let msg_ptr = Pointer::release_buffer(msg_data);

        let res_ptr = do_process_msg(
            &|_: Context, _: i32| -> Result<(), i32> { Err(0x0e) },
            msg_ptr.as_u64(),
        );

        let res_data = unsafe { Pointer::from_u64(res_ptr).to_slice() };
        assert_eq!(res_data, vec![0x82, 0x01, 0x0e]); // Result::Err(0x0e) -> http://cbor.me/?bytes=82010e
    }

    #[wasm_bindgen_test]
    fn test_query() {
        let msg_data = vec![0x00]; // http://cbor.me/?bytes=8100
        let msg_ptr = Pointer::release_buffer(msg_data);

        let res_ptr = do_query(
            &|_: Context, _: i32| -> Result<&str, i32> { Ok("foo") },
            msg_ptr.as_u64(),
        );

        let res_data = unsafe { Pointer::from_u64(res_ptr).to_slice() };
        assert_eq!(res_data, vec![0x82, 0x00, 0x63, 0x66, 0x6f, 0x6f]); // Result::Ok("foo") -> http://cbor.me/?bytes=0x820063666f6f
    }

    #[wasm_bindgen_test]
    fn test_query_error() {
        let msg_data = vec![0x00]; // http://cbor.me/?bytes=8100
        let msg_ptr = Pointer::release_buffer(msg_data);

        let res_ptr = do_query(
            &|_: Context, _: i32| -> Result<&str, i32> { Err(0x0e) },
            msg_ptr.as_u64(),
        );

        let res_data = unsafe { Pointer::from_u64(res_ptr).to_slice() };
        assert_eq!(res_data, vec![0x82, 0x01, 0x0e]); // Result::Err(0x0e) -> http://cbor.me/?bytes=82010e
    }
}
