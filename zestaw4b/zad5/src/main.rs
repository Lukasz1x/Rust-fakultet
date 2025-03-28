fn dodaj(a: char, b: char, c: &mut bool) -> char
{
    let num1: u8 = a as u8 - b'0';
    let num2: u8 = b as u8 - b'0';
    let num3: u8 = if *c {*c =false; 1} else {0};
    if num1+num2+num3 > 9
    {
        *c=true;
        // println!("{}+{}+{}={},  {}", num1, num2, num3,num1+num2+num3, (num1+num2+num3-10 + b'0') as char);
        return ((num1+num2+num3-10) + b'0') as char;
    }
    // println!("{}+{}+{}={},  {}", num1, num2, num3,num1+num2+num3, (num1+num2+num3 + b'0') as char);
    (num1+num2+num3 + b'0') as char
}

fn dodaj_pisemnie(a: &str, b: &str) -> String
{
    let mut l1 :Vec<char> = a.chars().rev().collect();
    let mut l2 :Vec<char> = b.chars().rev().collect();
    let mut result :Vec<char> = vec![];
    let mut res = String::new();

    let mut c:bool = false;
    // println!("{l1:?}");
    // println!("{l2:?}");

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
    // println!("{l1:?}");
    // println!("{l2:?}");
    for i in 0..l1.len()
    {
        result.push(dodaj(l1[i], l2[i], &mut c));
    }
    if c
    {
        result.push('1');
    }
    //println!("{result:?}");

    for x in result
    {
        res.push(x);
    }
    // println!("{}", res.chars().rev().collect::<String>());
    res.chars().rev().collect()
}

fn main() {
    println!("{}", dodaj_pisemnie("1", "3") == "4");
    println!("{}", dodaj_pisemnie("1", "3") == "4");
    println!("{}", dodaj_pisemnie("8", "3") == "11");
    println!("{}", dodaj_pisemnie("10", "23") == "33");
    println!("{}", dodaj_pisemnie("1", "0") == "1");
    println!("{}", dodaj_pisemnie("11", "00") == "11");
    println!("{}", dodaj_pisemnie("131", "9900") == "10031");
    println!("{}", dodaj_pisemnie("998", "7") == "1005");
    println!("{}", dodaj_pisemnie("24872947", "294729478") == "319602425");
    println!("{}", dodaj_pisemnie("5924729874298749827418582", "6782893629472094209740298") == "12707623503770844037158880");
}
