use std::cmp::Ordering;
use rand::Rng;

#[macro_use]
#[path = "macros/input_data.rs"]
mod input_data; 

fn main() {
    
    println!("ADIVINHE O NÚMERO ENTRE 1 E 99\n");
    
    let secret_number = rand::thread_rng()
    .gen_range(1..=99);


    let mut guesses: u32 =0;

    loop {
    
        let guess = input_data!(i32, "Digite um número: ");
    
        guesses += 1;

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("\nVocê ganhou!!!");
                break;
            },
            Ordering::Greater => println!("\nEstá para baixo!\n"),
            Ordering::Less => println!("\nEstá para cima!\n"),
        }
        
    }

    println!("Você tentou {} vezes até acertar", guesses)

}