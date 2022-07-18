use crate::error::Error;
use kelk::alloc::string::String;
use kelk::context::Context;
use kelk::kelk_derive;

#[kelk_derive(instantiate)]
pub fn instantiate(ctx: Context, _: ()) -> Result<(), Error> {
    Ok(ctx.storage.write_string(0, "hello world!", 16)?)
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
mod tests {
    use super::*;
    use kelk::alloc::string::ToString;
    use kelk::mock::mock_context;

    #[test]
    fn test_instantiate() {
        let ctx = mock_context(16);
        instantiate(ctx.as_ref(), ()).unwrap();
        let msg = query(ctx.as_ref(), ()).unwrap();
        assert_eq!(msg, "hello world!".to_string());
    }

    #[test]
    fn test_process() {
        let ctx = mock_context(16);
        instantiate(ctx.as_ref(), ()).unwrap();
        process(ctx.as_ref(), "foo".to_string()).unwrap();
        let msg = query(ctx.as_ref(), ()).unwrap();
        assert_eq!(msg, "foo".to_string());
    }
}
