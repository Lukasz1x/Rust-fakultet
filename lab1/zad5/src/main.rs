fn main() {
    let h1 :i32=7;
    let m1 :i32=16;
    let s1 :i32=50;

    let h2 :i32=6;
    let m2 :i32=59;
    let s2 :i32=02;

    let ss1 =s1+m1*60+h1*3600;
    let ss2 =s2+m2*60+h2*3600;

    //println!("{ss1}, {ss2}");
    let r = ss1-ss2;

    let s3 = r%60;
    let m3 =(r-s3)/60;
    let h3 = (r-s3-m3*60)/3600;

    println!("{h3}:{m3}:{s3}");
}
