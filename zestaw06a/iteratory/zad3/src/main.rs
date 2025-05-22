fn indeksy(tablica: &Vec<&str>, element: &str) -> Vec<usize>
{
    tablica.iter().zip(0..).filter(|z| *z.0 == element).map(|z| z.1 as usize ).collect()
}

fn main() {
    let test_strings = vec![
        "kot", "pies", "dom", "Ala", "zamek", "król", "kot", "robot", "1234", "kot", "test1",
        "inny", "pizza", "kot", "brutto", "kot", "lekki", "dzienny", "kot", "programowanie", "Rust",
        "wow", "gamma", "kot", "delta", "epsilon", "hello123", "abcd", "xyz", "foo", "bar",
    ];
    println!("{:?}", indeksy(&test_strings, "kot"));

}
