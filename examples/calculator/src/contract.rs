use crate::error::CalcError;
use crate::message::{ProcMsg, QueryMsg, QueryRsp};
use kelk_env::context::Context;

fn add(ctx: Context, a: i32, b: i32) -> Result<(), CalcError> {
    ctx.api
        .swrite_i32(0, a + b)
        .map_err(|_| CalcError::KelkError)
}

fn sub(ctx: Context, a: i32, b: i32) -> Result<(), CalcError> {
    ctx.api
        .swrite_i32(0, a - b)
        .map_err(|_| CalcError::KelkError)
}

fn mul(ctx: Context, a: i32, b: i32) -> Result<(), CalcError> {
    ctx.api
        .swrite_i32(0, a * b)
        .map_err(|_| CalcError::KelkError)
}

fn div(ctx: Context, a: i32, b: i32) -> Result<(), CalcError> {
    if b == 0 {
        return Err(CalcError::DivByZero);
    }
    ctx.api
        .swrite_i32(0, a / b)
        .map_err(|_| CalcError::KelkError)
}

fn query_result(ctx: Context) -> Result<i32, CalcError> {
    ctx.api.sread_i32(0).map_err(|_| CalcError::KelkError)
}

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

#[cfg(target_arch = "wasm32")]
mod __wasm_export_query {
    #[no_mangle]
    extern "C" fn query(msg_ptr: u64) -> u64 {
        kelk_env::do_query(&super::query, msg_ptr)
    }
}

// #[kelk_derive(instantiate)]
pub fn instantiate(_ctx: Context) -> Result<(), CalcError> {
    Ok(())
}

/// _ctx: Context) The process_msg function is the main function of the *deployed* contract actor
// #[kelk_derive(process_msg)]
pub fn process_msg(ctx: Context, msg: ProcMsg) -> Result<(), CalcError> {
    match msg {
        ProcMsg::Add { a, b } => add(ctx, a, b),
        ProcMsg::Sub { a, b } => sub(ctx, a, b),
        ProcMsg::Mul { a, b } => mul(ctx, a, b),
        ProcMsg::Div { a, b } => div(ctx, a, b),
    }
}

pub fn query(ctx: Context, msg: QueryMsg) -> Result<QueryRsp, CalcError> {
    let res = match msg {
        QueryMsg::LastResult => query_result(ctx),
    }?;

    Ok(QueryRsp::Result { res })
}

#[cfg(test)]
#[path = "./contract_test.rs"]
mod contract_test;
