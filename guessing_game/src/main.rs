use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("歡迎來到 *猜數字遊戲* ");

    let secret_num = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_num}");

    loop {
        println!("輸入你猜的數字");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("輸入數字: {guess}");
    
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("太小"),
            Ordering::Greater => println!("太大"),
            Ordering::Equal => {
                println!("中獎");
                break;
            },
        }
    }
}
