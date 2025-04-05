// wektor zawierającą napisy krótsze niż 4 znaki
fn fun1(v: Vec<&str>) -> Vec<String>
{
    v.iter().filter(|x|x.len()<4).map(|x| x.to_string()).collect()
}
// wektor napisów niezawierających liter 'a' ani 'A'
fn fun2(v: Vec<&str>) -> Vec<String>
{
    v.iter().filter(|x| !x.chars().any(|y|y=='a' || y=='A')).map(|x| x.to_string()).collect()
}
// wektor napisów zawierających cyfry
fn fun3(v: Vec<&str>) -> Vec<String>
{
    v.iter().filter(|x| x.chars().any(|y| y as u8 >= b'0' && y as u8 <=b'9')).map(|x| x.to_string()).collect()
}
// wektor zawierający te same napisy, ale odwrócone
fn fun4(v: Vec<&str>) -> Vec<String>
{
    v.iter().map(|x| x.chars().rev().collect()).collect()
}
// wektor napisów zawierających podwojoną literę (np.: inny, pizza, brutto, lekki, dzienny, itp.)
fn fun5(v: Vec<&str>) -> Vec<String>
{
    v.iter().filter(|x| x.chars().zip(x.chars().skip(1)).any(|(y,z)| y == z)).map(|x| x.to_string()).collect()
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
