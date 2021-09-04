use crate::error::CalcError;
use crate::message::CalcMsg;
use kelk::{context::ContextMut, kelk_derive, Response};

pub fn add(a: i32, b: i32) -> Result<i32, CalcError> {
    Ok(a + b)
}

pub fn sub(a: i32, b: i32) -> Result<i32, CalcError> {
    Ok(a - b)
}

pub fn mul(a: i32, b: i32) -> Result<i32, CalcError> {
    Ok(a * b)
}

pub fn div(a: i32, b: i32) -> Result<i32, CalcError> {
    if b == 0 {
        return Err(CalcError::DivByZero);
    }
    Ok(a / b)
}

/// The "instantiate" will be executed only once on instantiating the contract actor
#[cfg(target_arch = "wasm32")]
mod __wasm_export_instantiate {
    #[no_mangle]
    extern "C" fn instantiate() -> u32 {
        kelk::do_instantiate(&super::instantiate)
    }
}

#[cfg(target_arch = "wasm32")]
mod __wasm_export_process_msg {
    #[no_mangle]
    extern "C" fn process_msg(msg_ptr: *const u8, length: u32) -> u32 {
        kelk::do_process_msg(&super::process_msg, msg_ptr, length)
    }
}

// #[kelk_derive(instantiate)]
fn instantiate(Context: ContextMut) -> Result<Response, CalcError> {
    Ok(Response{
        res: 0,
    })
}

/// The process_msg function is the main function of the *deployed* contract actor
// #[kelk_derive(process_msg)]
fn process_msg(Context: ContextMut, msg: CalcMsg) -> Result<Response, CalcError> {
    let ans = match msg {
        CalcMsg::Add { a, b } => add(a, b),
        CalcMsg::Sub { a, b } => sub(a, b),
        CalcMsg::Mul { a, b } => mul(a, b),
        CalcMsg::Div { a, b } => div(a, b),
    }?;

    Ok(Response { res: ans })
}
