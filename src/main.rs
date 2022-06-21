use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Sayı tahmin oyununa hoş geldiniz!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    //println!("Gizli sayı: {}", secret_number);

    loop {

        println!("Bir sayı giriniz:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Girdiğiniz sayı:{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Girdiğiniz sayı fazla küçük!"),
            Ordering::Greater => println!("Girdiğiniz sayı fazla büyük!"),
            Ordering::Equal => {
                println!("Doğru tahmin!");
                break;
            }
        }
    }
}
