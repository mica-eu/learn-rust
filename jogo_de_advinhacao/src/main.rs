extern crate rand;

use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Adivinhe o número!");
    let numero_secreto: u32 = rand::thread_rng().gen_range(1..101);
    loop {
        println!("Digite o seu palpite.");
        let mut palpite: String = String::new();
        io::stdin()
            .read_line(&mut palpite)
            .expect("Falha ao ler a entrada.");
        let palpite: u32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Você disse: {}", palpite);
        match palpite.cmp(&numero_secreto) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("Você acertou!");
                break;
            }
        }
    }
}
