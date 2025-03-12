fn main() {
    let mut liczba = 123456789;
    while liczba != 0
    {
        print!("{} ",liczba%10);
        liczba /=10;
    }
}
