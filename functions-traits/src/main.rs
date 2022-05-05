#[allow(unused_imports)]
mod dev_impl;
mod animais;
mod animal_2;
use crate::animais::animal::*;
use crate::animal_2::animais_2::*;
use crate::dev_impl::dev::*;
// Macro
// Em macros podemos ter tipos de parâmetro: 
// expr, ident, block, stmt, pat, path, meta, ty e tt
macro_rules! name {
    ($name: expr) => (
        println!("My name is {}", $name);
    );
}
// macro com vários parâmetros
macro_rules! name {
    ($($name: expr), *) => { 
        $(println!("Hey {}", $name);)*
    };
}
// macro com variáveis
macro_rules! xy {
    (x => $e: expr) => {println!("X is {}", $e);};
    (y => $e: expr) => {println!("Y is {}", $e);};
}
// macro que constroi uma função
macro_rules! build_fn {
    ($fn_name: ident) => {
        fn $fn_name(){
            println!("{:?} this function was called", stringify!($fn_name));
        }
    };
}


fn main() {
    // Usando o macro
    name!("Kaio");
    xy!(x => 5);
    xy!(y => "Kaio");
    build_fn!(hey);
    hey();

    // struct JavaDev com o trait Developer
    let java_dev = JavaDev::new(true);
    println!("{}", java_dev.language());
    java_dev.say_hellow();

    // struct gato e cachorro com o trait latido
    let cachorro = Cachorro{species: "Pastor alemão"};
    let gato = Gato{cor: "Branco"};
    latido_teste(cachorro);
    latido_teste(gato);
    
    println!("{}", retorna_animal(-1.1).fazer_barulho());
}


