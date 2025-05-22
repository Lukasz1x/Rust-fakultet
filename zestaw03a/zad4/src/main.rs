fn swap_arr(arr: &mut [i32], i: usize, j: usize)
{
    let pom =arr[i];
    arr[i]=arr[j];
    arr[j]=pom;
}

fn main() {
    let mut tab0 = [1,4,90,34];

    println!("{:?}", tab0);
    swap_arr(&mut tab0, 0,1);
    println!("{:?}", tab0);
}