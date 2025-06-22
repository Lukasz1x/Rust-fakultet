use std::thread;

const ILE: u32 = 20;

fn main() {
    let mut watki = vec![];

    for i in 0..ILE {
        watki.push(thread::spawn(move || {
            println!("wątek: {}", i);
            println!("wątek: {}", i);
//             panic!();
            return i;
        }));
    }

    for watek in watki {
        let x = watek.join();
        println!("{:?}", x);
    }
}
