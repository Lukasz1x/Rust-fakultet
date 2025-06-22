macro_rules! witaj {
    () => {
        println!("Witaj, człowieku!")
    };
    ($x:expr) => {
        println!("Witaj, {}!", $x)
    };
    ($x:expr, $y:expr) => {
        println!("Witajcie, {} i {}!", $x, $y)
    };
}

fn main() {
    witaj!();
    witaj!("Edek");
    witaj!(123);
    witaj!("Jacek", "Placek");
}