pub trait Or {
    fn or(x: &Self, y: &Self) -> bool;
}

impl Or for bool {
    fn or(x: &Self, y: &Self) -> bool {
        *x || *y
    }
}

impl Or for char {
    fn or(x: &Self, y: &Self) -> bool {
        *x != '\0' || *y != '\0'
    }
}

impl Or for usize {
    fn or(x: &Self, y: &Self) -> bool {
        *x > 0 || *y > 0
    }
}

impl Or for u8 {
    fn or(x: &Self, y: &Self) -> bool {
        *x > 0 || *y > 0
    }
}

impl Or for u16 {
    fn or(x: &Self, y: &Self) -> bool {
        *x > 0 || *y > 0
    }
}

impl Or for u32 {
    fn or(x: &Self, y: &Self) -> bool {
        *x > 0 || *y > 0
    }
}

impl Or for u64 {
    fn or(x: &Self, y: &Self) -> bool {
        *x > 0 || *y > 0
    }
}

impl Or for u128 {
    fn or(x: &Self, y: &Self) -> bool {
        *x > 0 || *y > 0
    }
}

impl Or for isize {
    fn or(x: &Self, y: &Self) -> bool {
        *x > 0 || *y > 0
    }
}

impl Or for i8 {
    fn or(x: &Self, y: &Self) -> bool {
        *x > 0 || *y > 0
    }
}

impl Or for i16 {
    fn or(x: &Self, y: &Self) -> bool {
        *x > 0 || *y > 0
    }
}

impl Or for i32 {
    fn or(x: &Self, y: &Self) -> bool {
        *x > 0 || *y > 0
    }
}

impl Or for i64 {
    fn or(x: &Self, y: &Self) -> bool {
        *x > 0 || *y > 0
    }
}

impl Or for i128 {
    fn or(x: &Self, y: &Self) -> bool {
        *x > 0 || *y > 0
    }
}

impl Or for f32 {
    fn or(x: &Self, y: &Self) -> bool {
        *x > 0.0 || *y > 0.0
    }
}

impl Or for f64 {
    fn or(x: &Self, y: &Self) -> bool {
        *x > 0.0 || *y > 0.0
    }
}
