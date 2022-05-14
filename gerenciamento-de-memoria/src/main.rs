use std::vec;
use std::rc::Rc;

struct Car{
    brand: Rc<String>
}
impl Car{
    fn new(brand: Rc<String>) -> Car { return Car{ brand: brand}}
    fn drive(&self){
        println!("{} está dirigindo", &self.brand);
    }
}

#[derive(Debug)]
struct Person{
    name: String
}

// definindo um tempo de vida para a referência Person usando o "'l" ou "'lifetime".
// Significa que a referência Person vai ter o mesmo temp ode vida que a struct
#[derive(Debug)]
struct Dog<'lifetime>{
    name: String,
    owner: &'lifetime Person
}

impl Person {
    fn get_name(&self) -> &String{
        return &self.name;
    }
}
fn main() {
    //________________________________________Ownership__________________________________________
    // Para tipos primitivos como i32, acontece um copy de i para j. 
    // E i e j são independente uam da outra
    let i = 5;
    let j = i;
    println!("{}", i);
    println!("{}", j);

    // Variáveis do tipo complexo como vetores, não acontece um copy e sim uma tranferência de propriedade
    // de uma variável para outra.
    // Nesse exato caso, existe uma tranferência de C para W, logo  C não será mais acessível, pois fpo passado a propriedade para W.
    let c = vec![1, 2, 3];
    // let w = c;
    // println!("{:?}", c);
    // println!("{:?}", w);

    // Nesse caso a propriedade é transferida de C para a função anònima, e foi retransferida para C através do retorno da função.
    let foo = |c: Vec<i32>| -> Vec<i32>{
        println!("Vetor usado em foo");
        return c
    };
    let c = foo(c);
    println!("{:?}", c); 

    //________________________________________Borrowing__________________________________________
    // & serve para pegarmos emprestado a propriedade
    // Após o uso a variáveis que pegou emprestado é destruida.(Após a variável referenciada for chamada)
    // Para exemplificar melhor, coloca-se um 

    let mut a = 6;
    {
        let b = &mut a;
        println!("{}", *b);
        *b+=1;
        println!("{}", *b);
        *b+=1;
        println!("{}", *b);
    };
    println!("{}", a);
    let mut z = vec![1, 2, 3, 4];
    for elem in &z {
        println!("{}", elem);
    }
    //________________________________________Lifetimes__________________________________________
    let p1 = Person{name: String::from("Kaio")};
    let d1 = Dog{name: String::from("Estrela"), owner: &p1};

    let mut j: &String;
    {
        let p2 = Person{name: String::from("Kaio")};
        j = p1.get_name();
    };
    println!("{}", a);

    let brand = Rc::new(String::from("BMW"));
    println!("Pointers: {}", Rc::strong_count(&brand));
    {
        let car = Car::new(brand.clone());
        car.drive();
        println!("Pointers: {}", Rc::strong_count(&brand));
    }
    println!("Meu carro é uma {}", brand);
    println!("Pointers: {}", Rc::strong_count(&brand));
} 
