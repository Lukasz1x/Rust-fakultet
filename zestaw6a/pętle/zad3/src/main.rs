fn indeksy(tablica: &Vec<&str>, element: &str) -> Vec<usize>
{
    let mut res = vec![];
    for (i, x) in tablica.iter().enumerate()
    {
        if element == *x
        {
            res.push(i);
        }
    }

    res
}

fn main() {
    let test_strings = vec![
        "kot", "pies", "dom", "Ala", "zamek", "król", "kot", "robot", "1234", "kot", "test1",
        "inny", "pizza", "kot", "brutto", "kot", "lekki", "dzienny", "kot", "programowanie", "Rust",
        "wow", "gamma", "kot", "delta", "epsilon", "hello123", "abcd", "xyz", "foo", "bar",
    ];
    println!("{:?}", indeksy(&test_strings, "kot"));

}
