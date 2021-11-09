use crate::error::CalcError;
use crate::message::CalcMsg;
use kelk_env::{context::Context, Response};

fn add(_ctx: Context, a: i32, b: i32) -> Result<i32, CalcError> {
    Ok(a + b)
}

fn sub(_ctx: Context, a: i32, b: i32) -> Result<i32, CalcError> {
    Ok(a - b)
}

fn mul(_ctx: Context, a: i32, b: i32) -> Result<i32, CalcError> {
    Ok(a * b)
}

fn div(_ctx: Context, a: i32, b: i32) -> Result<i32, CalcError> {
    if b == 0 {
        return Err(CalcError::DivByZero);
    }
    Ok(a / b)
}

// pub fn set_memory(ctx: Context, m: i32) -> Result<i32, CalcError> {
//     let d = m.to_be_bytes();
//     ctx.write_storage(0, d)?;
//     Ok(m)
// }

// pub fn memory(ctx: Context) -> Result<i32, CalcError> {
//     let d = ctx.read_storage(0, 4)?;
//     let m = i32::from_be_bytes(d);
//     Ok(m)
// }

/// The "instantiate" will be executed only once on instantiating the contract actor
#[cfg(target_arch = "wasm32")]
mod __wasm_export_instantiate {
    #[no_mangle]
    extern "C" fn instantiate() -> u32 {
        kelk_env::do_instantiate(&super::instantiate)
    }
}

#[cfg(target_arch = "wasm32")]
mod __wasm_export_process_msg {
    #[no_mangle]
    extern "C" fn process_msg(msg_ptr: u64) -> u64 {
        kelk_env::do_process_msg(&super::process_msg, msg_ptr)
    }
}

// #[kelk_derive(instantiate)]
pub fn instantiate(_ctx: Context) -> Result<Response, CalcError> {
    Ok(Response { res: 0 })
}

/// _ctx: Context) The process_msg function is the main function of the *deployed* contract actor
// #[kelk_derive(process_msg)]
pub fn process_msg(ctx: Context, msg: CalcMsg) -> Result<Response, CalcError> {
    let ans = match msg {
        CalcMsg::Add { a, b } => add(ctx, a, b),
        CalcMsg::Sub { a, b } => sub(ctx, a, b),
        CalcMsg::Mul { a, b } => mul(ctx, a, b),
        CalcMsg::Div { a, b } => div(ctx, a, b),
    }?;

    Ok(Response { res: ans })
}

#[cfg(test)]
#[path = "./contract_test.rs"]
mod contract_test;
