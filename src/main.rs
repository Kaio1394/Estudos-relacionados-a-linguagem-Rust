mod archive;
use rand::Rng;
use crate::archive::arch::archive_file;
fn main() {
    // Aplicando importação de funções de arquivos externos
    let mut nome: String = String::from("Kaio");
    archive_file(nome);

    // Acessando funções dentro de um módulo. 
    //<nome do mod1>::<nome do mod2>::...<nome do modN>::<nome da função>
    let nomeKaio: String = String::from("Kaio");
    teste1::teste2::retorna_nome(nomeKaio);

    //Gerando número randômico através do pacote rand. 
    // Importação através do arquivo cargo.toml > [Dependencies] > rand = "0.8.4"
    let mut rng = rand::thread_rng();
    let a: i32 = rng.gen();
    println!("{}", a);
    
}
pub mod teste1{
    pub mod teste2{
        pub fn retorna_nome(nome: String) -> String{
            return nome;
        }
    }
}