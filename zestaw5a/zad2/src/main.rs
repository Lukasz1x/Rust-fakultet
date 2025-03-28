fn wartosc_syst2(z: &str) -> Option<u8>
{
    if z.len() == 0
    {
        return None;
    }
    let mut wynik:u8 =0;
    let mut dwa : u8 = 1;
    let mut bit8 =false;
    for x in z.chars().rev()
    {
        let znak = (x as i8)-(b'0' as i8);
        if znak != 0 && znak != 1
        {
            return None;
        }
        if znak == 1
        {
            if bit8
            {
                return None;
            }
            wynik += dwa;
        }
        if dwa < 128
        {
            dwa *= 2;
        }else {
            bit8 =true;
        }

    }

    Some(wynik)
}

fn main() {

    println!("{:?}", wartosc_syst2("00000001"));
    println!("{:?}", wartosc_syst2(""));
    println!("{:?}", wartosc_syst2("11111111"));
    println!("{:?}", wartosc_syst2("01010100"));
    println!("{:?}", wartosc_syst2("000010000001"));
    println!("{:?}", wartosc_syst2("100000001"));
}
