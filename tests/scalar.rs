use asserteq_pretty::assert_eq_pretty;

#[test]
#[should_panic(expected="assertion failed: `1` != `2`: u8 mismatch")]
fn test_u8_mismatch() {
    assert_eq_pretty!(1u8,2u8,"u8 mismatch")
}

#[test]
#[should_panic(expected="assertion failed: `1` != `2`: u16 mismatch")]
fn test_u16_mismatch() {
    assert_eq_pretty!(1u16,2u16,"u16 mismatch")
}

#[test]
#[should_panic(expected="assertion failed: `1` != `2`: u32 mismatch")]
fn test_u32_mismatch() {
    assert_eq_pretty!(1u32,2u32,"u32 mismatch")
}

#[test]
#[should_panic(expected="assertion failed: `1` != `2`: u64 mismatch")]
fn test_u64_mismatch() {
    assert_eq_pretty!(1u64,2u64,"u64 mismatch")
}

#[test]
#[should_panic(expected="assertion failed: `1` != `2`: u128 mismatch")]
fn test_u128_mismatch() {
    assert_eq_pretty!(1u64,2u64,"u128 mismatch")
}