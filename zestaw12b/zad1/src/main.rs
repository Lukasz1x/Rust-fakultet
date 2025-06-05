use zad1::ulamek::*;

fn main() {
    let mut u1 = Ulamek::new(5, 8);
    let mut u2 = Ulamek::new(43, 22);
    println!("{} + {} = {}", u1, u2, u1+u2);
    println!("{} - {} = {}", u1, u2, u1-u2);
    println!("{} * {} = {}", u1, u2, u1*u2);
    println!("{} / {} = {}", u1, u2, u1/u2);
    println!();

    println!("{} += {} ", u1, u2);
    u1+=u2;
    println!("    = {}", u1);

    println!("{} -= {} ", u1, u2);
    u1-=u2;
    println!("    = {}", u1);

    println!("{} *= {} ", u1, u2);
    u1*=u2;
    println!("    = {}", u1);

    println!("{} /= {} ", u1, u2);
    u1/=u2;
    println!("    = {}", u1);
    println!();

    println!("{} == {}  - {}",u1, u2, u1 == u2);
    println!("{} != {}  - {}",u1, u2, u1 != u2);

    println!("{} == {}  - {}",u1, u1, u1 == u1);
    println!("{} != {}  - {}",u2, u2, u2 != u2);

}
