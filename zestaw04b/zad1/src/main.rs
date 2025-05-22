fn co_drugi_znak(napis: &str) -> String
{
    let mut s :String="".to_string();
    for z in napis.chars().step_by(2)
    {
        s.push(z);
    }
    s
}

fn main() {
    let s:&str = "Hello, world!";
    println!("{}", co_drugi_znak(s));
}
