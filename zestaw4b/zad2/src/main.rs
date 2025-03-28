fn szyfruj(napis: &str, klucz: i32) -> String {
    let w1: Vec<char> = napis.chars().collect();
    let mut s1: String = "".to_string();
    for i in 0..((w1.len() as i32) / klucz) + 1 {
        for j in (i * klucz..if (i + 1) * klucz > w1.len() as i32 {
            w1.len() as i32
        } else {
            (i + 1) * klucz
        }).rev()
        {
            s1.push(w1[j as usize]);
        }
    }

    s1
}

fn main() {
    println!("{}", szyfruj("Aladyn", 2) == "lAdany");
    println!("{}", szyfruj("Aladyn", 3) == "alAnyd");
    println!("{}", szyfruj("Aladyn", 4) == "dalAny");
    println!("{}", szyfruj("Aladyn", 5) == "ydalAn");
    println!("{}", szyfruj("koza", 3) == "zoka");
    println!("{}", szyfruj("kaszanka", 3) == "saknazak");
    println!("{}", szyfruj("kot Mruczek", 9) == "zcurM tokke");
    println!("{}", szyfruj("kot Mruczek", 1) == "kot Mruczek");
    println!("{}", szyfruj("kot Mruczek", 2) == "ok trMcuezk");
}
