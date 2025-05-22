fn main() {
    // Małe litery alfabetu angielskiego
    let a :Vec<char> = (b'a'..=b'z').map(|x| x as char).collect();
    println!("{:?}", a);
    // Kwadraty 10. kolejnych liczb całkowitych począwszy od 1
    let b : Vec<i32> = (1..=10).map(|x|x*x).collect();
    println!("{:?}", b);
    // 10 kolejnych potęg dwójki
    let c : Vec<i32> = (1..=10).map(|x|2_i32.pow(x)).collect();
    println!("{:?}", c);
    // odwrotności wszystkich liczb od 1 do 20
    let d : Vec<f64> = (1..=20).map(|x|1.0/x as f64).collect();
    println!("{:?}", d);
    // liczby od 1 do 100 podzielne przez 3, ale niepodzielne przez 4
    let e : Vec<i32> = (1..=100).filter(|x| x%3==0 && x%4!=0).collect();
    println!("{:?}", e);
}
