mod condicional;
mod enumarate;
mod loops;
#[allow(unused_imports)]
use crate::loops::loop_test::{get_square, get_squareLoop};
use rand::{Rng, thread_rng};
use crate::enumarate::enum_teste::{print_choice, Suit, country, get_oranges};
use crate::condicional::condic::{verifica_numero, verifica_numero_menos_linha_codigo};

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
        println!("Sum = {}", elem);
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

    // Usando a variável global
    unsafe{
        R = 10;
        println!("R: {}", R);
    }

    // Funções anônimas
    let a = |a: i32, b: i32| -> i32 {
        return a + b
    };
    println!("{}", a(1, 2));

    let b = |x: i32| {println!("{}", x)};
    b(10);

    // 1: Somar todos valores de isq < limit, desde que isq seja um número par
    let limit = 200;
    let mut sum = 0;
    
    for i in 0..{
        let isq = i * i;
        if isq > limit {break;}
        else{
            if is_even(isq){
                sum += isq;
            }
        }
        println!("{0} * {0} = {1}", i, i * i);
    }
    println!("Sum = {}", sum);
    
    // 2: Outra forma de resolver
    let sum1 = (0..)
                .map(|x| x * x)
                .take_while(|&x| x <= limit)
                .filter(|x| is_even(*x))
                .fold(0, |sum, x| sum + x);
    
    println!("Sum1: {}", sum1);


    let mut x = 0;
    loop {
        let isq = x * x;
        if isq > limit {break;}
        else{
            if is_even(x){
                sum += isq;
            }
            x += 1;
        }
    }
    println!("Sum = {}", sum);

    // let mult = 
    //                 |a: &mut i32, b: &mut i32| &a * &b;
    // let mut number1: i32 = 1;
    // let mut number2: i32 = 1;
    //apply(mult, &mut number1, &mut number2);


    // Usando o macro
    name!("Kaio");
    xy!(x => 5);
    xy!(y => "Kaio");
    build_fn!(hey);
    hey();
}
fn is_even(x: i32) -> bool{
    x % 2 == 0
}



fn apply(f: fn(&mut i32, &mut i32) -> i32, 
                a: &mut i32, 
                b: &mut i32){
    loop{
        if f(a, b) < 500 {
            break;
        }else {
            println!("F: {}, a: {}, b: {}", f(a, b), &a, &b);
            *a += 1;
            *b += 1;
        }
    }
}


