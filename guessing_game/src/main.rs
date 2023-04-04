use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // cargo doc --open < baixa documentacao local do projeto e abre em um browser
    let secret_number = rand::thread_rng().gen_range(1..=3);

    // println!("Secret number: {secret_number}");

    loop { 

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Função Parse retonra um Result, que é um Enum. Então, o match verifica se o Enum é dos valores Ok ou Err, listados abaixo e executa o que for. 
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        
    }

}
