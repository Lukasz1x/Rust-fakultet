fn avg(tab: &[u32]) -> f32
{
    tab.iter().sum::<u32>() as f32 / tab.len() as f32
}

fn main()
{
    let v=Vec::from([5,35,5,45,23,35,356, 8798]);
    println!("{}", avg(&v));
}
