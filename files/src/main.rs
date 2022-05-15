mod  manipulacao_arquivo;
#[allow(unused_imports)]
use std::fs::File;
#[allow(unused_imports)]
use std::io::{Write, Read};
use std::fs::{OpenOptions, remove_file, rename};

use crate::manipulacao_arquivo::arquivo::*;

#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    //Criação de um arquivo txt ou rs
    criacao_codigo_rust("fn main() {\n   println!(\"\");  \n }", criacao_arquivo_rs(""));
    
    // Escrever em um arquivo repetidas vezes sem resetar o arquivo
    let mut file = OpenOptions::new().append(true)
        .open("src/example.txt")
        .expect("Arquivo não encontrado");
    file.write_all("fn main() {\n   println!(\"\");  \n }".as_bytes()).expect("Código não escrito");
    
    // Lendo arquivo 
    let file_read = let_arquivo("src/example.txt");
    println!("{}", file_read);

    // Renomeando arquivo
    renomeando_arquivo("src/kaio.rs", "src/teste.rs");


    // Removendo um arquivo
    
}

