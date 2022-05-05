#[allow(unused_imports)]
mod dev_impl;
mod animais;
mod animal_2;
mod adicionando_trait_a_uma_estrutura;
mod trait_existente;

use crate::adicionando_trait_a_uma_estrutura::*;
use crate:: trait_existente::trait_existente::*;
use crate::adicionando_trait_a_uma_estrutura::adicionando_trait::Somatorio;
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
    
    // Função que tem como retorno um trait.
    println!("{}", retorna_animal(-1.1).fazer_barulho());
    
    let mut vetor: Vec<i32> = Vec::new();
    vetor.push(5);
    vetor.push(5);
    vetor.push(5);
    println!("{:?}", vetor);

    // Trait somatório send aplicado em uma estrutura do tipo Vec
    println!("Soma = {}", vetor.soma());

    // Trait Add aplicado na estrutura Ponteiro
    let p1 = Ponteiro{X: 1.0, Y: 5.6};
    let p2 = Ponteiro{X: 5.1, Y: 10.6};
    let p3 = p1 + p2;
    println!("{:?}", p3);
}


