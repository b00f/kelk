use super::*;
use kelk::mock::mock_context;
use kelk::alloc::string::ToString;

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
