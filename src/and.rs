pub trait And {
    fn and(x: &Self, y: &Self) -> bool;
}

impl And for bool {
    fn and(x: &Self, y: &Self) -> bool {
        *x && *y
    }
}

impl And for char {
    fn and(x: &Self, y: &Self) -> bool {
        *x != '\0' && *y != '\0'
    }
}

impl And for usize {
    fn and(x: &Self, y: &Self) -> bool {
        *x > 0 && *y > 0
    }
}

impl And for u8 {
    fn and(x: &Self, y: &Self) -> bool {
        *x > 0 && *y > 0
    }
}

impl And for u16 {
    fn and(x: &Self, y: &Self) -> bool {
        *x > 0 && *y > 0
    }
}

impl And for u32 {
    fn and(x: &Self, y: &Self) -> bool {
        *x > 0 && *y > 0
    }
}

impl And for u64 {
    fn and(x: &Self, y: &Self) -> bool {
        *x > 0 && *y > 0
    }
}

impl And for u128 {
    fn and(x: &Self, y: &Self) -> bool {
        *x > 0 && *y > 0
    }
}

impl And for isize {
    fn and(x: &Self, y: &Self) -> bool {
        *x > 0 && *y > 0
    }
}

impl And for i8 {
    fn and(x: &Self, y: &Self) -> bool {
        *x > 0 && *y > 0
    }
}

impl And for i16 {
    fn and(x: &Self, y: &Self) -> bool {
        *x > 0 && *y > 0
    }
}

impl And for i32 {
    fn and(x: &Self, y: &Self) -> bool {
        *x > 0 && *y > 0
    }
}

impl And for i64 {
    fn and(x: &Self, y: &Self) -> bool {
        *x > 0 && *y > 0
    }
}

impl And for i128 {
    fn and(x: &Self, y: &Self) -> bool {
        *x > 0 && *y > 0
    }
}

impl And for f32 {
    fn and(x: &Self, y: &Self) -> bool {
        *x > 0.0 && *y > 0.0
    }
}

impl And for f64 {
    fn and(x: &Self, y: &Self) -> bool {
        *x > 0.0 && *y > 0.0
    }
}
