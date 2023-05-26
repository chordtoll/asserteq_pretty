use asserteq_pretty::assert_eq_pretty;

#[test]
fn test_t2_match() {
    assert_eq_pretty!((1, 2), (1, 2), "tuple mismatch")
}

#[test]
#[should_panic(
    expected = "assertion failed: Differences:\n\tPosition 0: `1` != `3`: tuple mismatch"
)]
fn test_t2a_mismatch() {
    assert_eq_pretty!((1, 2), (3, 2), "tuple mismatch")
}

#[test]
#[should_panic(
    expected = "assertion failed: Differences:\n\tPosition 1: `2` != `3`: tuple mismatch"
)]
fn test_t2b_mismatch() {
    assert_eq_pretty!((1, 2), (1, 3), "tuple mismatch")
}

#[test]
#[should_panic(
    expected = "assertion failed: Differences:\n\tPosition 0: `1` != `3`\n\tPosition 1: `2` != `4`: tuple mismatch"
)]
fn test_t2ab_mismatch() {
    assert_eq_pretty!((1, 2), (3, 4), "tuple mismatch")
}

#[test]
#[should_panic(
    expected = "assertion failed: Differences:\n\tPosition 2: `3` != `4`: tuple mismatch"
)]
fn test_t3_mismatch() {
    assert_eq_pretty!((1, 2, 3), (1, 2, 4), "tuple mismatch")
}

#[test]
#[should_panic(
    expected = "assertion failed: Differences:\n\tPosition 11: `12` != `13`: tuple mismatch"
)]
fn test_t12_mismatch() {
    assert_eq_pretty!(
        (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12),
        (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13),
        "tuple mismatch"
    )
}

#[test]
#[should_panic(
    expected = "assertion failed: Differences:\n\tPosition 0: `1` != `3`\n\tPosition 1: `'a'` != `'b'`: tuple mismatch"
)]
fn test_t2_nonhom() {
    assert_eq_pretty!((1, 'a'), (3, 'b'), "tuple mismatch")
}
