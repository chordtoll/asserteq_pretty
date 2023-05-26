use asserteq_pretty::assert_eq_pretty;
use asserteq_pretty::PrettyDiff;
use asserteq_pretty_macros::PrettyDiff;

#[derive(PrettyDiff, PartialEq, Debug)]
struct Foo {
    a: u8,
    b: u16,
}

#[test]
#[should_panic(
    expected = "assertion failed: Foo:\n\tField `a` differs: `1` != `3`: struct mismatch"
)]
fn test_s2a_mismatch() {
    assert_eq_pretty!(Foo { a: 1, b: 2 }, Foo { a: 3, b: 2 }, "struct mismatch")
}

#[test]
#[should_panic(
    expected = "assertion failed: Foo:\n\tField `b` differs: `2` != `3`: struct mismatch"
)]
fn test_s2b_mismatch() {
    assert_eq_pretty!(Foo { a: 1, b: 2 }, Foo { a: 1, b: 3 }, "struct mismatch")
}

#[test]
#[should_panic(
    expected = "assertion failed: Foo:\n\tField `a` differs: `1` != `3`\n\tField `b` differs: `2` != `4`: struct mismatch"
)]
fn test_s2ab_mismatch() {
    assert_eq_pretty!(Foo { a: 1, b: 2 }, Foo { a: 3, b: 4 }, "struct mismatch")
}
