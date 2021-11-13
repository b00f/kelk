use super::*;
use kelk_env::mock::mock_context;

#[test]
fn test_add() {
    let ctx = mock_context(10);
    add(ctx.as_ref(), 1, 2).unwrap();
    let res = query_result(ctx.as_ref()).unwrap();
    assert_eq!(res, 3);
}

#[test]
fn test_sub() {
    let ctx = mock_context(10);
    sub(ctx.as_ref(), 1, 2).unwrap();
    let res = query_result(ctx.as_ref()).unwrap();
    assert_eq!(res, -1);
}

#[test]
fn test_mul() {
    let ctx = mock_context(10);
    mul(ctx.as_ref(), 2, 2).unwrap();
    let res = query_result(ctx.as_ref()).unwrap();
    assert_eq!(res, 4);
}

#[test]
fn test_div() {
    let ctx = mock_context(10);
    div(ctx.as_ref(), 4, 2).unwrap();
    let res = query_result(ctx.as_ref()).unwrap();
    assert_eq!(res, 2);

    assert!(div(ctx.as_ref(), 4, 0).is_err());
}
