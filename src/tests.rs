use crate::Correct;

#[test]
fn it_works() {
    let mut a = Correct(1u8);
    a += 1;
    assert_eq!(a, 2);
    assert_eq!(a << 2, 8u8);
    assert_eq!(a << 8, 0u8);
}
