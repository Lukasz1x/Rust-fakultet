fn main() {
    let max =20;

    for a in 1..max-1
    {
        for b in a+1..max
        {
            for c in b+1..max+1
            {
                if a*a+b*b==c*c
                {
                    println!("0< {}<{}<{} <{}",a,b,c, max);
                }
            }
        }
    }
}