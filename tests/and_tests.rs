use logic_units::and::And;

#[test]
fn char_tt() {
    assert!(And::and(&'c', &'t'));
}

#[test]
fn char_ft() {
    assert!(!And::and(&'\0', &'t'));
}

#[test]
fn char_ff() {
    assert!(!And::and(&'\0', &'\0'));
}

#[test]
fn bool_tt() {
    assert!(And::and(&true, &true));
}

#[test]
fn bool_ft() {
    assert!(!And::and(&false, &true));
}

#[test]
fn bool_ff() {
    assert!(!And::and(&false, &false));
}

#[test]
fn usize_tt() {
    assert!(And::and(&10usize, &15usize));
}

#[test]
fn usize_ft() {
    assert!(!And::and(&0usize, &15usize));
}

#[test]
fn usize_ff() {
    assert!(!And::and(&0usize, &0usize));
}

#[test]
fn u8_tt() {
    assert!(And::and(&10u8, &15u8));
}

#[test]
fn u8_ft() {
    assert!(!And::and(&0u8, &15u8));
}

#[test]
fn u8_ff() {
    assert!(!And::and(&0u8, &0u8));
}

#[test]
fn u16_tt() {
    assert!(And::and(&10u16, &15u16));
}

#[test]
fn u16_ft() {
    assert!(!And::and(&0u16, &15u16));
}

#[test]
fn u16_ff() {
    assert!(!And::and(&0u16, &0u16));
}

#[test]
fn u32_tt() {
    assert!(And::and(&10u32, &15u32));
}

#[test]
fn u32_ft() {
    assert!(!And::and(&0u32, &15u32));
}

#[test]
fn u32_ff() {
    assert!(!And::and(&0u32, &0u32));
}

#[test]
fn u64_tt() {
    assert!(And::and(&10u64, &15u64));
}

#[test]
fn u64_ft() {
    assert!(!And::and(&0u64, &15u64));
}

#[test]
fn u64_ff() {
    assert!(!And::and(&0u64, &0u64));
}

#[test]
fn u128_tt() {
    assert!(And::and(&10u128, &15u128));
}

#[test]
fn u128_ft() {
    assert!(!And::and(&0u128, &15u128));
}

#[test]
fn u128_ff() {
    assert!(!And::and(&0u128, &0u128));
}

#[test]
fn isize_tt() {
    assert!(And::and(&10isize, &15isize));
}

#[test]
fn isize_ft() {
    assert!(!And::and(&0isize, &15isize));
}

#[test]
fn isize_ff() {
    assert!(!And::and(&0isize, &0isize));
}

#[test]
fn i8_tt() {
    assert!(And::and(&10i8, &15i8));
}

#[test]
fn i8_ft() {
    assert!(!And::and(&0i8, &15i8));
}

#[test]
fn i8_ff() {
    assert!(!And::and(&0i8, &0i8));
}

#[test]
fn i16_tt() {
    assert!(And::and(&10i16, &15i16));
}

#[test]
fn i16_ft() {
    assert!(!And::and(&0i16, &15i16));
}

#[test]
fn i16_ff() {
    assert!(!And::and(&0i16, &0i16));
}

#[test]
fn i32_tt() {
    assert!(And::and(&10i32, &15i32));
}

#[test]
fn i32_ft() {
    assert!(!And::and(&0i32, &15i32));
}

#[test]
fn i32_ff() {
    assert!(!And::and(&0i32, &0i32));
}

#[test]
fn i64_tt() {
    assert!(And::and(&10i64, &15i64));
}

#[test]
fn i64_ft() {
    assert!(!And::and(&0i64, &15i64));
}

#[test]
fn i64_ff() {
    assert!(!And::and(&0i64, &0i64));
}

#[test]
fn i128_tt() {
    assert!(And::and(&10i128, &15i128));
}

#[test]
fn i128_ft() {
    assert!(!And::and(&0i128, &15i128));
}

#[test]
fn i128_ff() {
    assert!(!And::and(&0i128, &0i128));
}

#[test]
fn f32_tt() {
    assert!(And::and(&1f32, &1f32));
}

#[test]
fn f32_ft() {
    assert!(!And::and(&0f32, &1f32));
}

#[test]
fn f32_ff() {
    assert!(!And::and(&0f32, &0f32));
}

#[test]
fn f64_tt() {
    assert!(And::and(&1f64, &1f64));
}

#[test]
fn f64_ft() {
    assert!(!And::and(&0f64, &1f64));
}

#[test]
fn f64_ff() {
    assert!(!And::and(&0f64, &0f64));
}
