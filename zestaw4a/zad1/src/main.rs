fn liczba_wystapien(napis: &str, znak: char) -> i32
{
    let mut licznik : i32 = 0;
    for z in napis.chars()
    {
        if z == znak
        {
            licznik += 1;
        }
    }

    licznik
}

fn main() {
    let s0 :&str = "Ala ma kota i psa";

    println!("{}", liczba_wystapien(s0, 'a'));
}
