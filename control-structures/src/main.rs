mod condicional;
mod enumarate;
#[allow(unused_imports)]
use crate::enumarate::enum_teste::Suit::Heart;
use rand::{Rng, thread_rng};
use crate::enumarate::enum_teste::{print_choice, Suit, country, get_oranges};
use crate::condicional::condic::{verifica_numero, verifica_numero_menos_linha_codigo};

fn main(){
    // Condicional "if"
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(0..10);
    println!("{}", verifica_numero(number));
    println!("{}", verifica_numero_menos_linha_codigo(number));  
    print_choice(Suit::Club);
    print_choice(Suit::Heart);
    print_choice(Suit::Diamond);
    print_choice(Suit::Spade);
    println!("Country: {}", country(44));
    println!("Country: {}", country(101));
    println!("{}", get_oranges(8));


    let point = (6, 0);
    match point{
        (0, 0) => println!("Origin"),
        (x, 0) => println!("x axis ({}, 0)", x),
        (0, y) => println!("y axis (0, {})", y),
        _ => println!("invalid")
    }

    //loop
    for elem in 1..10 {
        println!("{}", elem);
    }
}


