# Zestaw 13a
Rozszerz zadanie z Zestawu 12b w następujący sposób.

##### 1.1 Utwórz projekt biblioteczny, zawierający moduł ulamek, który implementuje typ `Ulamek` reprezentujący ułamek zwykły. Zaimplementowane powinny być przynajmniej następujące funkcje:
- konstruktor: `new(licznik, mianownik)`
- akcesory: `as_f64()`, `licznik()`, `mianownik()`
- operatory: `+` `-` `*` `/` `+=` `-=` `*=` `/=` `==` `!=` (jako cechy!)
- konstruktor konwertujący: `from_str(ulamek_jako_napis)`
##### 1.2 Dodatkowo, moduł ten ma zawierać wyczerpujący podmoduł testowy, którego część jest poniżej (znaczenie makra `#[should_panic]` powinno być oczywiste :)):
```rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_f64() {
        let u1 = Ulamek::new(3, 4);
        assert_eq!(u1.as_f64(), 0.75);
    }

    #[test]
    fn test_add() {
        let u1 = Ulamek::new(1, 3);
        let u2 = Ulamek::new(1, 2);
        assert_eq!(u1 + u2, Ulamek::new(5, 6));
    }

    #[test]
    #[should_panic]
    fn test_zerowy_mianownik() {
        let _ = Ulamek::new(1, 0);
    }

    #[test]
    fn test_rozne_zapisy_tego_samego_ulamka() {
        assert_eq!(Ulamek::new(1, -3), Ulamek::new(-2, 6));
    }

    #[test]
    fn test_z_napisu_1() {
        let u1 = Ulamek::from_str("1/-3").unwrap();
        let u2 = Ulamek::from_str("-2/6").unwrap();
        assert_eq!(u1, u2);
        assert_eq!(u1, Ulamek::new(-1, 3));
    }

    #[test]
    fn test_z_napisu_2() {
        let u1 = Ulamek::from_str("13").unwrap();
        let u2 = Ulamek::from_str("-26/-2").unwrap();
        assert_eq!(u1, u2);
        assert_eq!(u1, Ulamek::new(13, 1));
    }

    #[test]
    #[should_panic]
    fn test_z_blednego_napisu_1() {
        let _ = Ulamek::from_str("x/-3").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_z_blednego_napisu_2() {
        let _ = Ulamek::from_str("1/3/5").unwrap();
    }

    #[test]
    #[should_panic]
    fn test_z_blednego_napisu_3() {
        let _ = Ulamek::from_str("/5").unwrap();
    }
}
```