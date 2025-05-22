fn main() {
    let rok = 1600;
    if rok%400==0
    {
        println!("To jest rok przestępny");
    }else if rok%100==0
    {
        println!("To nie jest rok przestępny");
    }else if rok%4==0
    {
        println!("To jest rok przestępny");
    }else
    {
        println!("To nie jest rok przestępny");
    }
}
