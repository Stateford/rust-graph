#[test]
fn test_code() {
    let x = || -> i32 {
        return 1 + 2; 
        };
    let y : i32 = 3;
    assert_eq!(x(), y);
}
