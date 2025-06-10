use crate::uzdrowiciel::Uzdrowiciel;

use std::cell::RefCell;

impl Uzdrowiciel {
    pub fn ulecz(&mut self, cel: &RefCell<Uzdrowiciel>, przywracane_zdrowie: u32, koszt: u32) {
        if self.wydaj_mane(koszt).is_some() {
            cel.borrow_mut().zmien_zdrowie(przywracane_zdrowie as i32);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test3() {
        let /*mut niepotrzebne bo wewnętrzna mutowalność*/ edek = RefCell::new(Uzdrowiciel::new("Edek", 10, 5));
        let mut felek = Uzdrowiciel::new("Felek", 20, 15);
        edek.borrow_mut().zmien_zdrowie(-3);
        felek.ulecz(&edek, 10, 7);
        dbg!(edek);
        dbg!(felek);
    }

    #[test]
    fn test4() {
        let /*mut niepotrzebne bo wewnętrzna mutowalność*/ edek = RefCell::new(Uzdrowiciel::new("Edek", 10, 5));
        edek.borrow_mut().zmien_zdrowie(-3);
        edek.borrow_mut().ulecz(&edek, 10, 7);
        dbg!(edek);
    }

    #[test]
    fn test7() {
        let /*mut niepotrzebne bo wewnętrzna mutowalność*/ herosi = vec![
            RefCell::new(Uzdrowiciel::new("Edek", 10, 5)),
            RefCell::new(Uzdrowiciel::new("Felek", 20, 15)),
            RefCell::new(Uzdrowiciel::new("Jola", 120, 45)),
        ];
        herosi[0].borrow_mut().zmien_zdrowie(-3);
        herosi[1].borrow_mut().ulecz(&herosi[0], 10, 7);
        dbg!(herosi);
    }
}
