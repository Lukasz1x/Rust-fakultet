#[derive(Debug)]
pub struct Uzdrowiciel {
    mana: u32,
    zdrowie_akt: u32,
    zdrowie_maks: u32,
    imie: String,
}

impl Uzdrowiciel {
    pub fn new(imie: &str, zdrowie: u32, mana: u32) -> Self {
        Self {
            imie: imie.to_string(),
            mana,
            zdrowie_akt: zdrowie,
            zdrowie_maks: zdrowie,
        }
    }

    pub const fn zdrowie_akt(&self) -> &u32 {
        &self.zdrowie_akt
    }

    pub const fn zdrowie_maks(&self) -> &u32 {
        &self.zdrowie_maks
    }

    pub const fn imie(&self) -> &String {
        &self.imie
    }

    pub const fn mana(&self) -> &u32 {
        &self.mana
    }

    pub fn wydaj_mane(&mut self, wartosc: u32) -> Option<()> {
        if self.mana >= wartosc {
            self.mana -= wartosc;
            Some(())
        } else {
            None
        }
    }

    pub fn zmien_zdrowie(&mut self, wartosc: i32) {
        let nowe_zdrowie = self.zdrowie_akt as i32 + wartosc;
        if nowe_zdrowie < 0 {
            self.zdrowie_akt = 0;
        } else if nowe_zdrowie > self.zdrowie_maks as i32 {
            self.zdrowie_akt = self.zdrowie_maks;
        } else {
            self.zdrowie_akt = nowe_zdrowie as u32;
        }
    }

    pub const fn czy_zyje(&self) -> bool {
        self.zdrowie_akt > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let edek = Uzdrowiciel::new("Edek", 10, 5);
        let felek = Uzdrowiciel::new("Felek", 20, 15);
        assert_eq!(edek.zdrowie_akt(), &10);
        assert_eq!(edek.zdrowie_maks(), &10);
        assert_eq!(edek.imie(), "Edek");
        assert_eq!(edek.mana(), &5);
        assert_eq!(felek.zdrowie_akt(), &20);
        assert_eq!(felek.zdrowie_maks(), &20);
        assert_eq!(felek.imie(), "Felek");
        assert_eq!(felek.mana(), &15);
        dbg!(edek);
        dbg!(felek);
    }

    #[test]
    fn test2() {
        let mut edek = Uzdrowiciel::new("Edek", 10, 5);
        let mut felek = Uzdrowiciel::new("Felek", 20, 15);
        edek.zmien_zdrowie(-3);
        felek.wydaj_mane(10);
        assert_eq!(edek.zdrowie_akt(), &7);
        assert_eq!(edek.zdrowie_maks(), &10);
        assert_eq!(edek.imie(), "Edek");
        assert_eq!(edek.mana(), &5);
        assert_eq!(felek.zdrowie_akt(), &20);
        assert_eq!(felek.zdrowie_maks(), &20);
        assert_eq!(felek.imie(), "Felek");
        assert_eq!(felek.mana(), &5);
        dbg!(edek);
        dbg!(felek);
    }

    #[test]
    fn test2a() {
        let mut edek = Uzdrowiciel::new("Edek", 10, 5);
        edek.zmien_zdrowie(-3);
        assert_eq!(edek.zdrowie_akt(), &7);
        assert!(edek.czy_zyje());
        dbg!(&edek);
        edek.zmien_zdrowie(-30);
        assert_eq!(edek.zdrowie_akt(), &0);
        assert!(!edek.czy_zyje());
        dbg!(edek);
    }

    #[test]
    fn test5() {
        let herosi = vec![
            Uzdrowiciel::new("Edek", 10, 5),
            Uzdrowiciel::new("Felek", 20, 15),
            Uzdrowiciel::new("Jola", 120, 45),
        ];
        assert_eq!(herosi[0].zdrowie_akt(), &10);
        assert_eq!(herosi[0].zdrowie_maks(), &10);
        assert_eq!(herosi[0].imie(), "Edek");
        assert_eq!(herosi[0].mana(), &5);
        assert_eq!(herosi[1].zdrowie_akt(), &20);
        assert_eq!(herosi[1].zdrowie_maks(), &20);
        assert_eq!(herosi[1].imie(), "Felek");
        assert_eq!(herosi[1].mana(), &15);
        assert_eq!(herosi[2].zdrowie_akt(), &120);
        assert_eq!(herosi[2].zdrowie_maks(), &120);
        assert_eq!(herosi[2].imie(), "Jola");
        assert_eq!(herosi[2].mana(), &45);
        dbg!(herosi);
    }

    #[test]
    fn test6() {
        let mut herosi = vec![
            Uzdrowiciel::new("Edek", 10, 5),
            Uzdrowiciel::new("Felek", 20, 15),
            Uzdrowiciel::new("Jola", 120, 45),
        ];
        herosi[0].zmien_zdrowie(-3);
        assert_eq!(herosi[0].zdrowie_akt(), &7);
        assert_eq!(herosi[0].zdrowie_maks(), &10);
        assert_eq!(herosi[0].imie(), "Edek");
        assert_eq!(herosi[0].mana(), &5);
        assert_eq!(herosi[1].zdrowie_akt(), &20);
        assert_eq!(herosi[1].zdrowie_maks(), &20);
        assert_eq!(herosi[1].imie(), "Felek");
        assert_eq!(herosi[1].mana(), &15);
        assert_eq!(herosi[2].zdrowie_akt(), &120);
        assert_eq!(herosi[2].zdrowie_maks(), &120);
        assert_eq!(herosi[2].imie(), "Jola");
        assert_eq!(herosi[2].mana(), &45);
        dbg!(herosi);
    }
}
