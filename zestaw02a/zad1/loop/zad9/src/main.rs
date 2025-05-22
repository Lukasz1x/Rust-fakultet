fn main() {
    let mut a :i32 =1;
    let mut b :i32 =2;
    let mut c :i32 =3;

    let max =20;

    loop
    {
        if b<=a
        {
            break;
        }
        b=a+1;
        loop
        {
            if c<=b
            {
                break;
            }
            c=b+1;
            loop
            {
                if max<c
                {
                    break;
                }
                if a*a+b*b==c*c
                {
                    println!("0< {}<{}<{} <{}",a,b,c, max);
                }
                c+=1;
            }
            b+=1;
        }
        a+=1;
    }
}