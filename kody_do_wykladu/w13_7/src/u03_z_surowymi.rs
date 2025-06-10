use crate::uzdrowiciel::Uzdrowiciel;

impl Uzdrowiciel {
    pub fn ulecz(&mut self, cel: *mut Uzdrowiciel, przywracane_zdrowie: u32, koszt: u32) {
        if self.wydaj_mane(koszt).is_some() {
            unsafe {
                cel.as_mut()
                    .expect("nie spodziewamy się tu błędu!")
                    .zmien_zdrowie(przywracane_zdrowie as i32);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test3() {
        let mut edek = Uzdrowiciel::new("Edek", 10, 5);
        let mut felek = Uzdrowiciel::new("Felek", 20, 15);
        edek.zmien_zdrowie(-3);
        felek.ulecz(&mut edek, 10, 7);
        dbg!(edek);
        dbg!(felek);
    }

    #[test]
    fn test4() {
        let mut edek = Uzdrowiciel::new("Edek", 10, 5);
        edek.zmien_zdrowie(-3);
        let edek_raw = &mut edek as *mut Uzdrowiciel;
        edek.ulecz(edek_raw, 10, 7);
        dbg!(edek);
    }

    #[test]
    fn test7() {
        let mut herosi = vec![
            Uzdrowiciel::new("Edek", 10, 5),
            Uzdrowiciel::new("Felek", 20, 15),
            Uzdrowiciel::new("Jola", 120, 45),
        ];
        herosi[0].zmien_zdrowie(-3);
        let edek_raw = &mut herosi[0] as *mut Uzdrowiciel;
        herosi[1].ulecz(edek_raw, 10, 7);
        dbg!(herosi);
    }
}
