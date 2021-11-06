use kelk_env::mock::MockContext;

use super::*;

#[test]
fn test_add() {
    let mut ctx = MockContext::new(10);
    let res = add(ctx.as_mut(), 1, 1).unwrap();
    assert_eq!(res, 2);
}
