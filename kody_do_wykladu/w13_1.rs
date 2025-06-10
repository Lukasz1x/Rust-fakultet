use std::collections::HashSet;

fn main() {
    let s1 = HashSet::new();
    let mut s2 = HashSet::from([1, 3, 2, 4, 3, 2, 1]);
    let s3 = HashSet::from([1, 3, 2, 4, 2, 1]);
    println!("{:?}", s1);
    println!("{:?}", s2);
    println!("{:?}", s3);
    println!("{:?}", s1 == s2);
    println!("{:?}", s3 == s2);
    s2.insert(10);
    println!("{:?}", s3 == s2);
    println!("{:?}", s3.is_subset(&s2));
    println!("{:?}", s2.is_superset(&s3));
}