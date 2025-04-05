// wektor zawierającą napisy krótsze niż 4 znaki
fn fun1(v: Vec<&str>) -> Vec<String>
{
    let mut res :Vec<String> = vec![];
    for i in v
    {
        if i.len() < 4
        {
            res.push(i.to_string());
        }
    }

    res

}
// wektor napisów niezawierających liter 'a' ani 'A'
fn fun2(v: Vec<&str>) -> Vec<String>
{
    let mut res :Vec<String> = vec![];
    for i in v
    {
        let mut a = true;
        for j in i.chars()
        {
            if j == 'a' || j == 'A'
            {
                a=false;
            }
        }
        if a
        {
            res.push(i.to_string());
        }
    }

    res

}
// wektor napisów zawierających cyfry
fn fun3(v: Vec<&str>) -> Vec<String>
{
    let mut res :Vec<String> = vec![];
    for i in v
    {
        let mut a = false;
        for j in i.chars()
        {
            if j as u8 >= b'0' && j as u8 <= b'9'
            {
                a=true;
                break;
            }
        }
        if a
        {
            res.push(i.to_string());
        }
    }

    res

}
// wektor zawierający te same napisy, ale odwrócone
fn fun4(v: Vec<&str>) -> Vec<String>
{
    let mut res :Vec<String> = vec![];
    for i in v
    {
        let mut a = String::new();
        for j in i.chars().rev()
        {
            a.push(j);
        }
        res.push(a);
    }

    res

}
// wektor napisów zawierających podwojoną literę (np.: inny, pizza, brutto, lekki, dzienny, itp.)
fn fun5(v: Vec<&str>) -> Vec<String>
{
    let mut res :Vec<String> = vec![];
    for i in v
    {
        let mut a = false;
        let mut poprzednia = i.chars().nth(0).unwrap_or(' ');
        for j in i.chars().skip(1)
        {
            if j == poprzednia
            {
                a=true;
                break;
            }
            poprzednia = j;
        }
        if a
        {
            res.push(i.to_string());
        }
    }

    res

}

fn main() {
    let test_strings = vec![
        "kot", "pies", "dom", "Ala", "zamek", "król", "robot", "1234", "test1",
        "inny", "pizza", "brutto", "lekki", "dzienny", "programowanie", "Rust",
        "wow", "gamma", "delta", "epsilon", "hello123", "abcd", "xyz", "foo", "bar",
    ];

    println!("{:?}", fun1(test_strings.clone()));
    println!("{:?}", fun2(test_strings.clone()));
    println!("{:?}", fun3(test_strings.clone()));
    println!("{:?}", fun4(test_strings.clone()));
    println!("{:?}", fun5(test_strings.clone()));
}
