fn wartosc_cyfry(c: char) -> Result<u8, String>
{
    let znak = (c as i8)-(b'0' as i8);
    if znak >9 || znak <0
    {
        return Err("Podany znak nie jest cyfrą!".to_string());
    }
    Ok(znak as u8)
}

fn dodaj_pisemnie(a: &str, b: &str) -> Result<String, String>
{
    let mut result :String = String::new();
    let mut l1 :Vec<char> = a.chars().rev().collect();
    let mut l2 :Vec<char> = b.chars().rev().collect();

    let mut c:bool = false;

    while l1.len() != l2.len()
    {
        if l1.len() > l2.len()
        {
            l2.push('0');
        }else
        {
            l1.push('0');
        }
    }
    for i in 0..l1.len()
    {
        let mut suma=wartosc_cyfry(l1[i])? + wartosc_cyfry(l2[i])?;
        if c
        {
            suma+=1;
        }
        if suma > 9
        {
            c=true;
            suma-=10;
        }else { c=false }
        result.push_str(suma.to_string().as_str());
    }
    if c
    {
        result.push('1');
    }

    // println!("{}", result.chars().rev().collect::<String>());
    Ok(result.chars().rev().collect())
}

fn main() {
    println!("{}", dodaj_pisemnie("1", "3").unwrap_or("-".to_string()) == "4");
    println!("{}", dodaj_pisemnie("1", "3").unwrap_or("-".to_string()) == "4");
    println!("{}", dodaj_pisemnie("8", "3").unwrap_or("-".to_string()) == "11");
    println!("{}", dodaj_pisemnie("10", "23").unwrap_or("-".to_string()) == "33");
    println!("{}", dodaj_pisemnie("1", "0").unwrap_or("-".to_string()) == "1");
    println!("{}", dodaj_pisemnie("11", "00").unwrap_or("-".to_string()) == "11");
    println!("{}", dodaj_pisemnie("131", "9900").unwrap_or("-".to_string()) == "10031");
    println!("{}", dodaj_pisemnie("998", "7").unwrap_or("-".to_string()) == "1005");
    println!("{}", dodaj_pisemnie("24872947", "294729478").unwrap_or("-".to_string()) == "319602425");
    println!("{}", dodaj_pisemnie("5924729874298749827418582", "6782893629472094209740298").unwrap_or("-".to_string()) == "12707623503770844037158880");
    println!("{:?}", dodaj_pisemnie("1dasf", "ssfs3"));
}
