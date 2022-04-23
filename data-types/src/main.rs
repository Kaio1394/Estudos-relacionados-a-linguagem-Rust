mod func_update;
mod struct_job;
mod enums;
mod generic;
use crate::enums::{Colors, ColorsGeneric};
use crate::struct_job::job::Employee;
use crate::generic::Generic::{Point, PointMultiple, PointNMultiple};
use crate::func_update::update::{update_colors, change_string};

#[allow(unused_assignments)]
fn main() {
    // setando o tipo do array e o tamanho
    // não pode ser adicionado novos elementos em um arrays
    // mas podem ser alterados
    const DEFAULT: i32 = 3;
    let numbers: [i32; 5] = [DEFAULT; 5];
    let array: [i32; 4] = [0, 1, 2, 3];
    let _doubles: [f64; 4] = [1.0, 2.0, 3.0, 4.0]; 
    println!("{}", array.len());
    println!("{:?}", array[0]); // pegando índice de um array
    println!("{:?}", numbers);
    // somatória dos elementos de um array
    let mut soma = 0;
    for i in 0..numbers.len() {
        soma += numbers[i];
        println!("{} -> {}", i, numbers[i]);
        numbers.iter();
    }
    println!("{}", soma);

    // Vetores
    // Pode ser adicionado novos elementos
    let mut primes: Vec<Vec<i32>> = Vec::new();
    primes = vec![vec![2, 3, 5], vec![2, 3, 5]];
    primes.push(vec![1, 2]);
    // Similar setup da estrutura dos arrays 
    // [Tipo do elemento(int, bool, int, etc...); N_vezes]
    let vetor = vec![2; 10];  
    println!("{:?}", primes);
    println!("{:?}", vetor);
    primes.remove(0); // Removendo um elemento do vetor pelo indice
    println!("{:?}", primes);

    // Slice
    let array_teste = [0, 1, 2, 3, 4, 5, 6];
    let slice_array = &array_teste[0..3];
    println!("{:?}", slice_array);

    let mut colors = ["red", "green", "blue", "pink"];
    println!("{:?}", colors);
    update_colors(&mut colors[2..4]);
    println!("{:?}", colors); 
    
    // Mudando um valor referenciado na memória usando o operador "*"
    let mut nome = "kaio";
    println!("{}", nome);
    change_string(&mut nome);
    println!("{}", nome);

    // Tuplas
    // Não pode adicionar e nem remover elementos
    let mut person: (&str, u64, bool) = ("Kaio", 27, true);
    // Imprimindo índice
    person.0 = "Yuri";
    println!("{}", person.0);
    // Nomeando os elementos da tupla
    let (name, age, employment) = person;
    println!("{}, {}, {}", name, age, employment);

    // Struct e impl
    let kaio: Employee = Employee{
        name: String::from("Kaio"),
        company: String::from("Vericode"),
        age: 27
    };
    println!("{}", kaio.fn_details());
    // Método estático não recebe o parâmetro &self
    println!("{}", Employee::static_fn_detail());
    
    // Acessando um enum
    let my_color = Colors::Red;
    println!("{:?}", my_color); 

    // Genérico
    let p1: Point<i32> = Point {x: 6, y: 10};
    let p2: Point<f32> = Point {x: 50.2, y: 10.2};
    println!("{}", p1.x);
    println!("{}", p2.x);
    // Enum genérico
    let c1 = ColorsGeneric::Black(String::from("#4FG2A"));
    println!("{:?}", c1);
    // múltiplos genéricos
    let p_multiplo: PointMultiple<i32, String> =  PointMultiple {
        x: 20, 
        y: String::from("Kaio")
    };
    let p_n_multiplo: PointNMultiple<u32, u32, u32> = PointNMultiple {
        x: 13,
        y: 27,
        c: 10
    };
    
}

