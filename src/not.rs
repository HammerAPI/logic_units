pub trait Not {
    fn not(x: &Self) -> Self;
}

impl Not for bool {
    fn not(x: &Self) -> Self {
        !x
    }
}

impl Not for char {
    fn not(x: &Self) -> Self {
        if *x == '\0' {
            ' '
        } else {
            '\0'
        }
    }
}

impl Not for usize {
    fn not(x: &Self) -> Self {
        if *x == 0 {
            1
        } else {
            0
        }
    }
}

impl Not for u8 {
    fn not(x: &Self) -> Self {
        if *x == 0 {
            1
        } else {
            0
        }
    }
}

impl Not for u16 {
    fn not(x: &Self) -> Self {
        if *x == 0 {
            1
        } else {
            0
        }
    }
}

impl Not for u32 {
    fn not(x: &Self) -> Self {
        if *x == 0 {
            1
        } else {
            0
        }
    }
}

impl Not for u64 {
    fn not(x: &Self) -> Self {
        if *x == 0 {
            1
        } else {
            0
        }
    }
}

impl Not for u128 {
    fn not(x: &Self) -> Self {
        if *x == 0 {
            1
        } else {
            0
        }
    }
}

impl Not for isize {
    fn not(x: &Self) -> Self {
        if *x == 0 {
            1
        } else {
            0
        }
    }
}

impl Not for i8 {
    fn not(x: &Self) -> Self {
        if *x == 0 {
            1
        } else {
            0
        }
    }
}

impl Not for i16 {
    fn not(x: &Self) -> Self {
        if *x == 0 {
            1
        } else {
            0
        }
    }
}

impl Not for i32 {
    fn not(x: &Self) -> Self {
        if *x == 0 {
            1
        } else {
            0
        }
    }
}

impl Not for i64 {
    fn not(x: &Self) -> Self {
        if *x == 0 {
            1
        } else {
            0
        }
    }
}

impl Not for i128 {
    fn not(x: &Self) -> Self {
        if *x == 0 {
            1
        } else {
            0
        }
    }
}

impl Not for f32 {
    fn not(x: &Self) -> Self {
        if *x == 0.0 {
            1.0
        } else {
            0.0
        }
    }
}

impl Not for f64 {
    fn not(x: &Self) -> Self {
        if *x == 0.0 {
            1.0
        } else {
            0.0
        }
    }
}
