fn rand(seed: &mut i64, min_rand: i64, max_rand: i64) -> i64
{
    let a =1664525;
    let c =1013904223;
    let m:i64=(2 as i64).pow(32);
    *seed =(a* *seed+c)%m;

    *seed%(max_rand-min_rand+1)+min_rand
}

fn swap_arr(arr: &mut [i32], i: usize, j: usize)
{
    let pom =arr[i];
    arr[i]=arr[j];
    arr[j]=pom;
}

fn rand_perm(arr: &mut [i32], seed: &mut i64)
{
    for _i in 0..arr.len()
    {
        let j = rand(seed, 0,arr.len() as i64) as usize;
        let k = rand(seed, 0,arr.len() as i64) as usize;
        swap_arr(arr, j,k);
    }
}


fn main() {
    let mut tab = [0 as i32;100];
    for i  in 0..100
    {
        tab[i] = i as i32;
    }
    println!("{:?}", tab);
    let mut seed =999;
    rand_perm(&mut tab, &mut seed);
    println!("{:?}", tab);

}
