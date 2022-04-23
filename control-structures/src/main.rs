mod condicional;
mod enumarate;
#[allow(unused_imports)]
use crate::enumarate::enum_teste::Suit::Heart;
use rand::{Rng, thread_rng};
use crate::enumarate::enum_teste::{print_choice, Suit};
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
}


