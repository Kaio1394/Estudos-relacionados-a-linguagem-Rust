#[allow(unused_imports)]
use rand::{Rng, thread_rng};

fn main() {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(0..10);
    if number >= 5{
        println!("Número {} é maior ou igual a 5", number);
    }else{
        println!("Número{} é menor ou igual a 5", number);
    }
}
