mod archive;
use rand::{Rng, thread_rng};
use rand::{distributions::Alphanumeric};
use crate::archive::arch::archive_file;
fn main() {
    // Aplicando importação de funções de arquivos externos
    let nome: String = String::from("Kaio");
    archive_file(nome);

    // Acessando funções dentro de um módulo. 
    //<nome do mod1>::<nome do mod2>::...<nome do modN>::<nome da função>
    let nome_kaio: String = String::from("Kaio");
    teste1::teste2::retorna_nome(nome_kaio);

    //Gerando número randômico através do pacote rand. 
    // Importação através do arquivo cargo.toml > [Dependencies] > rand = "0.8.4"
    let mut rng = rand::thread_rng();
    let a: i32 = rng.gen();
    println!("{}", a);
    // Setando um range para o número randômico. Tanto inteiro quanto float
    println!("{}", rng.gen_range(0.0..100.0));

    // Gerando uma string randômica
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(2)
        .map(char::from)
        .collect();
    println!("{}", rand_string);
}
pub mod teste1{
    pub mod teste2{
        pub fn retorna_nome(nome: String) -> String{
            return nome;
        }
    }
}