use crate::error::CalcError;
use crate::message::CalcMsg;
use kelk::{context::ContextMut, entry_point, export::Response};

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
#[no_mangle]
extern "C" fn instantiate() {}

/// The process function is the main function of the *deployed* contract actor
#[entry_point]
fn process(Context: ContextMut, msg: CalcMsg) -> Result<Response, CalcError> {
    let ans = match msg {
        CalcMsg::Add { a, b } => add(a, b),
        CalcMsg::Sub { a, b } => sub(a, b),
        CalcMsg::Mul { a, b } => mul(a, b),
        CalcMsg::Div { a, b } => div(a, b),
    }?;

    Ok(Response{res: ans})
}
