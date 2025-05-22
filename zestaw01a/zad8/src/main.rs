fn main() {
    let mut liczba = 123456789;
    let mut suma=0;
    while liczba != 0
    {
        suma+=liczba%10;
        liczba /=10;
    }
    print!("{suma}");
}
