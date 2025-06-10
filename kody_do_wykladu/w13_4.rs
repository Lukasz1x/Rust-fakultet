use std::collections::BTreeMap;

fn main() {
    let mut s1: BTreeMap<char, i32> = BTreeMap::new();
    s1.insert('a', 3);
    s1.insert('b', 33);
    s1.insert('c', 23);
    println!("{:?}", s1);
    s1.insert('a', 723);
    println!("{:?}", s1);
    for (k, v) in &s1 {
        println!("{k}: {v}");
    }
    println!("{:?}", s1.entry('a'));
    println!("{:?}", s1.entry('x'));
    
    s1.entry('b').or_insert(145);
    s1.entry('y').or_insert(745);
    println!("{:?}", s1);

    s1.entry('a').and_modify(|a| (*a)*=1000);
    s1.entry('z').and_modify(|a| (*a)*=-1000);
    println!("{:?}", s1);
}