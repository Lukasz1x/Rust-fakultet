fn main()
{
    let n =100;
    let mut tab :Vec<i32> = vec![];
    for i in 1..=n
    {
        tab.push(100%i);
    }
    tab.iter().rev().for_each(|x|print!("{} ", x));
}
