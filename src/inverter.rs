pub trait Inverter {
    fn invert(x: &Self) -> Self;
}

impl Inverter for bool {
    fn invert(x: &Self) -> Self {
        !x
    }
}

impl Inverter for char {
    fn invert(x: &Self) -> Self {
        if *x == '\0' {
            ' '
        } else {
            '\0'
        }
    }
}

impl Inverter for usize {
    fn invert(x: &Self) -> Self {
        if *x == 0 {
            1
        } else {
            0
        }
    }
}

impl Inverter for u8 {
    fn invert(x: &Self) -> Self {
        if *x == 0 {
            1
        } else {
            0
        }
    }
}

impl Inverter for u16 {
    fn invert(x: &Self) -> Self {
        if *x == 0 {
            1
        } else {
            0
        }
    }
}

impl Inverter for u32 {
    fn invert(x: &Self) -> Self {
        if *x == 0 {
            1
        } else {
            0
        }
    }
}

impl Inverter for u64 {
    fn invert(x: &Self) -> Self {
        if *x == 0 {
            1
        } else {
            0
        }
    }
}

impl Inverter for u128 {
    fn invert(x: &Self) -> Self {
        if *x == 0 {
            1
        } else {
            0
        }
    }
}

impl Inverter for isize {
    fn invert(x: &Self) -> Self {
        if *x == 0 {
            1
        } else {
            0
        }
    }
}

impl Inverter for i8 {
    fn invert(x: &Self) -> Self {
        if *x == 0 {
            1
        } else {
            0
        }
    }
}

impl Inverter for i16 {
    fn invert(x: &Self) -> Self {
        if *x == 0 {
            1
        } else {
            0
        }
    }
}

impl Inverter for i32 {
    fn invert(x: &Self) -> Self {
        if *x == 0 {
            1
        } else {
            0
        }
    }
}

impl Inverter for i64 {
    fn invert(x: &Self) -> Self {
        if *x == 0 {
            1
        } else {
            0
        }
    }
}

impl Inverter for i128 {
    fn invert(x: &Self) -> Self {
        if *x == 0 {
            1
        } else {
            0
        }
    }
}

impl Inverter for f32 {
    fn invert(x: &Self) -> Self {
        if *x == 0.0 {
            1.0
        } else {
            0.0
        }
    }
}

impl Inverter for f64 {
    fn invert(x: &Self) -> Self {
        if *x == 0.0 {
            1.0
        } else {
            0.0
        }
    }
}
