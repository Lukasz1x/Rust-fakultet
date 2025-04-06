fn fun(napis: &str) -> String
{
    napis.chars().step_by(2).collect()
}

fn main() {
    println!("{}", fun("Hello, world!"));
}
