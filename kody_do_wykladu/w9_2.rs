struct S(u8, u8);

impl PartialEq for S {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl PartialOrd for S {
    fn partial_cmp(&self, other: &Self)
                   -> Option<std::cmp::Ordering> {
        self.1.partial_cmp(&other.1)
    }
}

fn main() {
    let s1 = S(1, 5);
    let s2 = S(2, 1);
    let s3 = S(3, 1);
    println!("{}", s1 == s2);   // false
    println!("{}", s1 == s3);   // false
    println!("{}", s3 == s2);   // true
    println!("{}", s1 < s2);    // false
    println!("{}", s1 < s3);    // false
    println!("{}", s3 < s2);    // false
}
