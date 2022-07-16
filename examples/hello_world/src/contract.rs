use crate::error::Error;
use kelk::context::Context;
use kelk::kelk_derive;
use kelk::alloc::string::String;

#[kelk_derive(instantiate)]
pub fn instantiate(ctx: Context, _: ()) -> Result<(), Error> {
    Ok(ctx.storage.write_string(0,"hello world!",  16)?)
}

#[kelk_derive(process)]
pub fn process(ctx: Context, msg: String) -> Result<(), Error> {
    Ok(ctx.storage.write_string(0, &msg, 16)?)
}

#[kelk_derive(query)]
pub fn query(ctx: Context, _: ()) -> Result<String, Error> {
    Ok(ctx.storage.read_string(0, 16)?)
}

#[cfg(test)]
#[path = "./contract_test.rs"]
mod contract_test;
