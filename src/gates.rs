use crate::{and::And, gates, inverter::Inverter, or::Or};

pub fn xor<T: And + Or + Inverter>(x: T, y: T) -> bool {
    let a = Inverter::invert(&And::and(&x, &y));
    let b = Or::or(&x, &y);
    return And::and(&a, &b);
}
