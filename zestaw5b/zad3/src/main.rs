fn wartosc_cyfry_rzymskiej(c: char) -> Result<u16, String>
{

    let rzymskie = [('I', 1),  ('V', 5), ('X', 10), ('L', 50), ('C', 100), ('D', 500), ('M', 1000)];
    for i in rzymskie
    {
        if c == i.0
        {
            return  Ok(i.1);
        }
    }

    Err("Podany znak nie jest cyfrą rzymską!".to_string())
}

fn main() {

    println!("{:?}", wartosc_cyfry_rzymskiej('I'));
    println!("{:?}", wartosc_cyfry_rzymskiej('V'));
    println!("{:?}", wartosc_cyfry_rzymskiej('X'));
    println!("{:?}", wartosc_cyfry_rzymskiej('L'));
    println!("{:?}", wartosc_cyfry_rzymskiej('C'));
    println!("{:?}", wartosc_cyfry_rzymskiej('D'));
    println!("{:?}", wartosc_cyfry_rzymskiej('M'));
    println!("{:?}", wartosc_cyfry_rzymskiej('i'));
    println!("{:?}", wartosc_cyfry_rzymskiej('h'));
    println!("{:?}", wartosc_cyfry_rzymskiej('3'));
    println!("{:?}", wartosc_cyfry_rzymskiej('?'));
}
