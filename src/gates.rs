use crate::{and::And, not::Not, or::Or};

pub fn and<T: And>(x: T, y: T) -> bool {
    And::and(&x, &y)
}

pub fn or<T: Or>(x: T, y: T) -> bool {
    Or::or(&x, &y)
}

pub fn not<T: Not>(x: T, y: T) -> T {
    Not::not(&x)
}

pub fn xor<T: And + Or + Not>(x: T, y: T) -> bool {
    let a = Not::not(&And::and(&x, &y));
    let b = Or::or(&x, &y);
    And::and(&a, &b)
}

pub fn nor<T: Or>(x: T, y: T) -> bool {
    Not::not(&Or::or(&x, &y))
}

pub fn nand<T: And>(x: T, y: T) -> bool {
    Not::not(&And::and(&x, &y))
}

pub fn xnor<T: And + Or + Not>(x: T, y: T) -> bool {
    let a = Not::not(&And::and(&x, &y));
    let b = Or::or(&x, &y);
    Not::not(&And::and(&a, &b))
}
