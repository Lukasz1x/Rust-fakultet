fn krotsze_niz_4(wej: Vec<String>) -> Vec<String> {
    wej.iter()
        .filter(|x| x.len() < 4)
        .map(|x| (*x).clone())
        .collect()
}

fn bez_liter_a(wej: Vec<String>) -> Vec<String> {
    wej.into_iter()
        .filter(|x| !x.chars().any(|y| y == 'a' || y == 'A'))
        .collect()
}

fn zawierajace_cyfry(wej: Vec<String>) -> Vec<String> {
    wej.iter()
        .filter(|x| x.chars().any(|y| y.is_ascii_digit()))
        .map(|x| (*x).clone())
        .collect()
}

fn palindromy(wej: Vec<String>) -> Vec<String> {
    wej.iter()
        .map(|x| (x.chars(), x.chars().rev()))
        .filter(|x| (x.0.clone()).zip(x.1.clone()).all(|y| y.0 == y.1))
        .map(|(a, b)| a.collect())
        .collect()
}

fn z_podwojeniem_litery(wej: Vec<String>) -> Vec<String> {
    wej.iter()
        .filter(|x| {
            x.chars().enumerate().any(|(i, c)| {
                (i > 0 && x.chars().nth(i - 1).unwrap() == c)
                    || (i < x.chars().count() - 1 && x.chars().nth(i + 1).unwrap() == c)
            })
        })
        .map(|z| (*z).clone())
        .collect()
}

fn main() {
    let dane: Vec<String> = vec![
        "ok".to_string(),      // < 4 znaki
        "dom".to_string(),     // < 4 znaki, bez 'a', bez cyfr, nie palindrom
        "kajak".to_string(),   // palindrom
        "Anna".to_string(), // palindrom (jeśli nie zważasz na wielkość), zawiera 'A', podwójne 'n'
        "wanna".to_string(), // podwojenie 'n'
        "test123".to_string(), // zawiera cyfry
        "miasto".to_string(), // zawiera 'a'
        "kot".to_string(),  // < 4 znaki
        "R2D2".to_string(), // zawiera cyfry
        "Noon".to_string(), // palindrom (jeśli ignorujemy wielkość liter)
        "xyzzyx".to_string(), // palindrom
        "panna".to_string(), // podwójne 'n'
        "echo".to_string(), // nie zawiera a/A, bez cyfr, niepalindrom
        "ALA".to_string(),  // zawiera 'A'
        "12321".to_string(), // cyfry i palindrom
    ];
    println!("krotsze_niz_4: {:?}", krotsze_niz_4(dane.clone()));
    println!("bez_liter_a: {:?}", bez_liter_a(dane.clone()));
    println!("zawierajace_cyfry: {:?}", zawierajace_cyfry(dane.clone()));
    println!("palindromy: {:?}", palindromy(dane.clone()));
    println!(
        "z_podwojeniem_litery: {:?}",
        z_podwojeniem_litery(dane.clone())
    );
}
