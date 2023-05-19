use asserteq_pretty::assert_eq_pretty;

#[test]
fn test_t2_match() {
    assert_eq_pretty!((1,2),(1,2),"tuple mismatch")
}

#[test]
#[should_panic(expected="assertion failed: Differences:\n\tPos 0: `1` != `3`: tuple mismatch")]
fn test_t2a_mismatch() {
    assert_eq_pretty!((1,2),(3,2),"tuple mismatch")
}

#[test]
#[should_panic(expected="assertion failed: Differences:\n\tPos 1: `2` != `3`: tuple mismatch")]
fn test_t2b_mismatch() {
    assert_eq_pretty!((1,2),(1,3),"tuple mismatch")
}

#[test]
#[should_panic(expected="assertion failed: Differences:\n\tPos 0: `1` != `3`\n\tPos 1: `2` != `4`: tuple mismatch")]
fn test_t2ab_mismatch() {
    assert_eq_pretty!((1,2),(3,4),"tuple mismatch")
}