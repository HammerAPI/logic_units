use logic_units::{and::And, gates, inverter::Inverter, or::Or};

fn main() {
    andit(2, 1);
    andit('c', 'b');
    andit('c', '\0');
    andit(0.2, 1.0);
    andit(false, true);
    orit(2, 1);
    orit('c', 'b');
    orit('c', '\0');
    orit(0.2, 1.0);
    orit(false, true);
    invertit(2);
    invertit(0);
    invertit('c');
    invertit('\0');
    invertit(1.0);
    invertit(0.0);
    invertit(true);
    invertit(false);
}

fn andit<T: std::fmt::Display + And>(x: T, y: T) {
    print!("{} AND {} | ", x, y);
    println!("RES: {}", And::and(&x, &y));
}

fn orit<T: std::fmt::Display + Or>(x: T, y: T) {
    print!("{} OR {} | ", x, y);
    println!("RES: {}", Or::or(&x, &y));
}

fn invertit<T: std::fmt::Display + Inverter>(x: T) {
    print!("NOT {} ", x);
    println!("RES: {}", Inverter::invert(&x));
}
