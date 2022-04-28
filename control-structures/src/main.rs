mod condicional;
mod enumarate;
mod loops;
#[allow(unused_imports)]
use crate::loops::loop_test::{get_square, get_squareLoop};
use rand::{Rng, thread_rng};
use crate::enumarate::enum_teste::{print_choice, Suit, country, get_oranges};
use crate::condicional::condic::{verifica_numero, verifica_numero_menos_linha_codigo};

// Variável global
static mut R: i32 = 0;

fn main(){
    // Condicional "if"
    let mut rng = thread_rng();
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

    let pets = ["dog", "cat", "hamster"];
    for elem in pets.iter() {
        if elem == &"cat"{
            println!("Achou o {}", elem);
            continue;
        }
        println!("I love my {}", elem);
    }

    for(pos, i) in (1..11).enumerate(){
        let square = i * i;
        let number = pos + 1;
        println!("{0} * {0} = {1}", number, square);
    }

    get_square(3151);
    get_squareLoop(3151);

    unsafe{
        R = 10;
        println!("R: {}", R);
    }

}





