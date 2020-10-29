use logic_units::or::Or;

#[test]
fn char_tt() {
    assert!(Or::or(&'c', &'t'));
}

#[test]
fn char_ft() {
    assert!(Or::or(&'\0', &'t'));
}

#[test]
fn char_ff() {
    assert!(!Or::or(&'\0', &'\0'));
}

#[test]
fn bool_tt() {
    assert!(Or::or(&true, &true));
}

#[test]
fn bool_ft() {
    assert!(Or::or(&false, &true));
}

#[test]
fn bool_ff() {
    assert!(!Or::or(&false, &false));
}

#[test]
fn usize_tt() {
    assert!(Or::or(&10usize, &15usize));
}

#[test]
fn usize_ft() {
    assert!(Or::or(&0usize, &15usize));
}

#[test]
fn usize_ff() {
    assert!(!Or::or(&0usize, &0usize));
}

#[test]
fn u8_tt() {
    assert!(Or::or(&10u8, &15u8));
}

#[test]
fn u8_ft() {
    assert!(Or::or(&0u8, &15u8));
}

#[test]
fn u8_ff() {
    assert!(!Or::or(&0u8, &0u8));
}

#[test]
fn u16_tt() {
    assert!(Or::or(&10u16, &15u16));
}

#[test]
fn u16_ft() {
    assert!(Or::or(&0u16, &15u16));
}

#[test]
fn u16_ff() {
    assert!(!Or::or(&0u16, &0u16));
}

#[test]
fn u32_tt() {
    assert!(Or::or(&10u32, &15u32));
}

#[test]
fn u32_ft() {
    assert!(Or::or(&0u32, &15u32));
}

#[test]
fn u32_ff() {
    assert!(!Or::or(&0u32, &0u32));
}

#[test]
fn u64_tt() {
    assert!(Or::or(&10u64, &15u64));
}

#[test]
fn u64_ft() {
    assert!(Or::or(&0u64, &15u64));
}

#[test]
fn u64_ff() {
    assert!(!Or::or(&0u64, &0u64));
}

#[test]
fn u128_tt() {
    assert!(Or::or(&10u128, &15u128));
}

#[test]
fn u128_ft() {
    assert!(Or::or(&0u128, &15u128));
}

#[test]
fn u128_ff() {
    assert!(!Or::or(&0u128, &0u128));
}

#[test]
fn isize_tt() {
    assert!(Or::or(&10isize, &15isize));
}

#[test]
fn isize_ft() {
    assert!(Or::or(&0isize, &15isize));
}

#[test]
fn isize_ff() {
    assert!(!Or::or(&0isize, &0isize));
}

#[test]
fn i8_tt() {
    assert!(Or::or(&10i8, &15i8));
}

#[test]
fn i8_ft() {
    assert!(Or::or(&0i8, &15i8));
}

#[test]
fn i8_ff() {
    assert!(!Or::or(&0i8, &0i8));
}

#[test]
fn i16_tt() {
    assert!(Or::or(&10i16, &15i16));
}

#[test]
fn i16_ft() {
    assert!(Or::or(&0i16, &15i16));
}

#[test]
fn i16_ff() {
    assert!(!Or::or(&0i16, &0i16));
}

#[test]
fn i32_tt() {
    assert!(Or::or(&10i32, &15i32));
}

#[test]
fn i32_ft() {
    assert!(Or::or(&0i32, &15i32));
}

#[test]
fn i32_ff() {
    assert!(!Or::or(&0i32, &0i32));
}

#[test]
fn i64_tt() {
    assert!(Or::or(&10i64, &15i64));
}

#[test]
fn i64_ft() {
    assert!(Or::or(&0i64, &15i64));
}

#[test]
fn i64_ff() {
    assert!(!Or::or(&0i64, &0i64));
}

#[test]
fn i128_tt() {
    assert!(Or::or(&10i128, &15i128));
}

#[test]
fn i128_ft() {
    assert!(Or::or(&0i128, &15i128));
}

#[test]
fn i128_ff() {
    assert!(!Or::or(&0i128, &0i128));
}

#[test]
fn f32_tt() {
    assert!(Or::or(&1f32, &1f32));
}

#[test]
fn f32_ft() {
    assert!(Or::or(&0f32, &1f32));
}

#[test]
fn f32_ff() {
    assert!(!Or::or(&0f32, &0f32));
}

#[test]
fn f64_tt() {
    assert!(Or::or(&1f64, &1f64));
}

#[test]
fn f64_ft() {
    assert!(Or::or(&0f64, &1f64));
}

#[test]
fn f64_ff() {
    assert!(!Or::or(&0f64, &0f64));
}
