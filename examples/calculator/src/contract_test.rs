use super::*;

#[test]
fn test_add() {
    let ctx = MockContext(); // TODO
    let res = add(ctx, 1, 1).unwrap();
    assert_eq!(res, 2);
}
