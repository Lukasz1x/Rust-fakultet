fn main() {
    // małe litery alfabetu angielskiego
    let mut v0 :Vec<char> = vec![];
    for i in b'a'..=b'z'
    {
        v0.push(i as char);
    }
    println!("{:?}", v0);
    // kwadraty 10. kolejnych liczb całkowitych począwszy od 1
    let mut v1 :Vec<i32> = vec![];
    for i in 1..=10
    {
        v1.push(i*i);
    }
    println!("{:?}", v1);
    // 10 kolejnych potęg dwójki
    let mut v2 :Vec<i32> = vec![];
    for i in 1..=10
    {
        v2.push(2_i32.pow(i));
    }
    println!("{:?}", v2);
    // odwrotności wszystkich liczb od 1 do 20
    let mut v3 :Vec<f64> = vec![];
    for i in 1..=20
    {
        v3.push(1.0/i as f64);
    }
    println!("{:?}", v3);
    // liczby od 1 do 100 podzielne przez 3, ale niepodzielne przez 4
    let mut v4 :Vec<i32> = vec![];
    for i in 1..=100
    {
        if i%3 == 0 && i%4 != 0
        {
            v4.push(i);
        }
    }
    println!("{:?}", v4);

}
