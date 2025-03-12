fn main() {
    let mut a :i32 =1;
    let mut b :i32 =2;
    let mut c :i32 =3;

    let max =100;

    while a<b
    {
        b=a+1;
        while b<c
        {
            c=b+1;
            while c<=max
            {
                if a*a+b*b==c*c
                {
                    println!("0<{}<{}<{}",a,b,c);
                }
                c+=1;
            }
            b+=1;
        }
        a+=1;
    }
}
