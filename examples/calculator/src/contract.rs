use crate::error::Error;
use crate::message::{ProcessMsg, QueryMsg, QueryRsp};
use kelk_env::kelk_derive;
use kelk_env::context::Context;

fn add(ctx: Context, a: i32, b: i32) -> Result<(), Error> {
    ctx.storage
        .write_i32(0, a + b)
        .map_err(|_| Error::KelkError)
}

fn sub(ctx: Context, a: i32, b: i32) -> Result<(), Error> {
    ctx.storage
        .write_i32(0, a - b)
        .map_err(|_| Error::KelkError)
}

fn mul(ctx: Context, a: i32, b: i32) -> Result<(), Error> {
    ctx.storage
        .write_i32(0, a * b)
        .map_err(|_| Error::KelkError)
}

fn div(ctx: Context, a: i32, b: i32) -> Result<(), Error> {
    if b == 0 {
        return Err(Error::DivByZero);
    }
    ctx.storage
        .write_i32(0, a / b)
        .map_err(|_| Error::KelkError)
}

fn query_result(ctx: Context) -> Result<i32, Error> {
    ctx.storage.read_i32(0).map_err(|_| Error::KelkError)
}

#[kelk_derive(process)]
pub fn process(ctx: Context, msg: ProcessMsg) -> Result<(), Error> {
    match msg {
        ProcessMsg::Add { a, b } => add(ctx, a, b),
        ProcessMsg::Sub { a, b } => sub(ctx, a, b),
        ProcessMsg::Mul { a, b } => mul(ctx, a, b),
        ProcessMsg::Div { a, b } => div(ctx, a, b),
    }
}

#[kelk_derive(query)]
pub fn query(ctx: Context, msg: QueryMsg) -> Result<QueryRsp, Error> {
    let res = match msg {
        QueryMsg::LastResult => query_result(ctx),
    }?;

    Ok(QueryRsp::Result { res })
}

#[cfg(test)]
#[path = "./contract_test.rs"]
mod contract_test;
