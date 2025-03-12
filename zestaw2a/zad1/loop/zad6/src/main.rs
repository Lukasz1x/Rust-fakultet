fn main() {
    let d =6;
    let mut i =1;
    let mut silnia=1;
    loop
    {
        if d<i{
            break;
        }
        silnia *= i;
        i+=1;
    }
    println!("{silnia}");
}