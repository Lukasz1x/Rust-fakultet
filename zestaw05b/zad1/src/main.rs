fn wartosc_cyfry(c: char) -> Result<u8, String>
{
    let znak = (c as i8)-(b'0' as i8);
    if znak >9 || znak <0
    {
        return Err("Podany znak nie jest cyfrą!".to_string());
    }
    Ok(znak as u8)
}

fn main() {
    println!("{:?}", wartosc_cyfry('0'));
    println!("{:?}", wartosc_cyfry('1'));
    println!("{:?}", wartosc_cyfry('2'));
    println!("{:?}", wartosc_cyfry('3'));
    println!("{:?}", wartosc_cyfry('4'));
    println!("{:?}", wartosc_cyfry('5'));
    println!("{:?}", wartosc_cyfry('6'));
    println!("{:?}", wartosc_cyfry('7'));
    println!("{:?}", wartosc_cyfry('8'));
    println!("{:?}", wartosc_cyfry('9'));
    println!("{:?}", wartosc_cyfry('g'));
    println!("{:?}", wartosc_cyfry('!'));
    println!("{:?}", wartosc_cyfry('-'));

}
