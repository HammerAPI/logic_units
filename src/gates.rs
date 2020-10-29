use crate::{and::And, gates, not::Not, or::Or};

pub fn xor<T: And + Or + Not>(x: T, y: T) -> bool {
    let a = Not::not(&And::and(&x, &y));
    let b = Or::or(&x, &y);
    return And::and(&a, &b);
}
