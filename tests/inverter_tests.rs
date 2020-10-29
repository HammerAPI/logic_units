use logic_units::inverter::Inverter;

#[test]
fn char_t() {
    assert_eq!(Inverter::invert(&'c'), '\0');
}

#[test]
fn char_f() {
    assert_eq!(Inverter::invert(&'\0'), ' ');
}

#[test]
fn bool_t() {
    assert_eq!(Inverter::invert(&true), false);
}

#[test]
fn bool_f() {
    assert_eq!(Inverter::invert(&false), true);
}

#[test]
fn usize_t() {
    assert_eq!(Inverter::invert(&15usize), 0);
}

#[test]
fn usize_f() {
    assert_eq!(Inverter::invert(&0usize), 1);
}

#[test]
fn u8_t() {
    assert_eq!(Inverter::invert(&11u8), 0);
}
#[test]
fn u8_f() {
    assert_eq!(Inverter::invert(&0u8), 1);
}

#[test]
fn u16_t() {
    assert_eq!(Inverter::invert(&11u16), 0);
}
#[test]
fn u16_f() {
    assert_eq!(Inverter::invert(&0u16), 1);
}

#[test]
fn u32_t() {
    assert_eq!(Inverter::invert(&11u32), 0);
}

#[test]
fn u32_f() {
    assert_eq!(Inverter::invert(&0u32), 1);
}

#[test]
fn u64_t() {
    assert_eq!(Inverter::invert(&11u64), 0);
}

#[test]
fn u64_f() {
    assert_eq!(Inverter::invert(&0u64), 1);
}

#[test]
fn u128_t() {
    assert_eq!(Inverter::invert(&11u128), 0);
}

#[test]
fn u128_f() {
    assert_eq!(Inverter::invert(&0u128), 1);
}

#[test]
fn isize_t() {
    assert_eq!(Inverter::invert(&11isize), 0);
}

#[test]
fn isize_f() {
    assert_eq!(Inverter::invert(&0isize), 1);
}

#[test]
fn i8_t() {
    assert_eq!(Inverter::invert(&11i8), 0);
}

#[test]
fn i8_f() {
    assert_eq!(Inverter::invert(&0i8), 1);
}

#[test]
fn i16_t() {
    assert_eq!(Inverter::invert(&11i16), 0);
}

#[test]
fn i16_f() {
    assert_eq!(Inverter::invert(&0i16), 1);
}

#[test]
fn i32_t() {
    assert_eq!(Inverter::invert(&11i32), 0);
}

#[test]
fn i32_f() {
    assert_eq!(Inverter::invert(&0i32), 1);
}

#[test]
fn i64_t() {
    assert_eq!(Inverter::invert(&11i64), 0);
}

#[test]
fn i64_f() {
    assert_eq!(Inverter::invert(&0i64), 1);
}

#[test]
fn i128_t() {
    assert_eq!(Inverter::invert(&11i128), 0);
}

#[test]
fn i128_f() {
    assert_eq!(Inverter::invert(&0i128), 1);
}

#[test]
fn f32_t() {
    assert_eq!(Inverter::invert(&11f32), 0.0);
}

#[test]
fn f32_f() {
    assert_eq!(Inverter::invert(&0f32), 1.0);
}

#[test]
fn f64_t() {
    assert_eq!(Inverter::invert(&11f64), 0.0);
}

#[test]
fn f64_f() {
    assert_eq!(Inverter::invert(&0f64), 1.0);
}
