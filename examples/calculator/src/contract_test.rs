use kelk_env::mock::mock_context;

use super::*;

#[test]
fn test_add() {
    let ctx = mock_context(10);
    let res = add(ctx.as_ref(), 1, 1).unwrap();
    assert_eq!(res, 2);
}
