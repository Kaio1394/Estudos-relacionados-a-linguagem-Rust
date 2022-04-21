mod condicional;
#[allow(unused_imports)]
use rand::{Rng, thread_rng};
use crate::condicional::condic::{verifica_numero, verifica_numero_menos_linha_codigo};

fn main() {
    // Condicional "if"
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(0..10);
    println!("{}", verifica_numero(number));
    println!("{}", verifica_numero_menos_linha_codigo(number));
}
