#[allow(unused_imports)]
mod dev_impl;
mod animais;
use crate::animais::*;
use crate::animais::animal::*;
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

// Implementando uma função com o retoro de uma trait através do Box<dyn ...>
struct Papagaio{}
struct Cobra{}
trait Animal_2{
    fn fazer_barulho(&self) -> &'static str;
}
impl Animal_2 for Papagaio{
    fn fazer_barulho(&self) -> &'static str {
        todo!()
    }
}
impl Animal_2 for Cobra{
    fn fazer_barulho(&self) -> &'static str {
        todo!()
    }
}
fn retorna_animal(numero: f32) -> Box<dyn Animal_2>{
    if numero < 0.0 {
        return Box::new(Papagaio{})
    }else{
        return Box::new(Cobra{})
    }
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
    
}


