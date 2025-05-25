fn main() {
    let a:Vec<char> = ('a'..='z').collect();
    println!("{a:?}");
    let b:Vec<i32> = (1..=10).map(|x| x*x).collect();
    println!("{b:?}");
    let c:Vec<i32> = (1..=10).map(|x| 2_i32.pow(x)).collect();
    println!("{c:?}");
    let d:Vec<f64> = (1..=20).map(|x| 1.0/x as f64).collect();
    println!("{d:?}");
    let e:Vec<i32> = (1..=100).filter(|x| x%3==0 && x%4!=0).collect();
    println!("{e:?}");
}
