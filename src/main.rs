use logic_units::{and::And, gates, not::Not, or::Or};

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
    notit(2);
    notit(0);
    notit('c');
    notit('\0');
    notit(1.0);
    notit(0.0);
    notit(true);
    notit(false);
}

fn andit<T: std::fmt::Display + And>(x: T, y: T) {
    print!("{} AND {} | ", x, y);
    println!("RES: {}", And::and(&x, &y));
}

fn orit<T: std::fmt::Display + Or>(x: T, y: T) {
    print!("{} OR {} | ", x, y);
    println!("RES: {}", Or::or(&x, &y));
}

fn notit<T: std::fmt::Display + Not>(x: T) {
    print!("NOT {} ", x);
    println!("RES: {}", Not::not(&x));
}
