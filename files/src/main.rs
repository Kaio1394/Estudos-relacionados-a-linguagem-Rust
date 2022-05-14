mod  manipulacao_arquivo;
use crate::manipulacao_arquivo::arquivo::*;
#[allow(unused_imports)]
#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    //Criação de um arquivo txt ou rs
    criacao_codigo_txt("Kaio", criacao_arquivo_txt(""));
    // let mut file = OpenOptions::new().append(true)
    //     .open("src/example.txt")
    //     .expect("Arquivo não encontrado");
    // file.write_all("println!(\"\");".as_bytes()).expect("Código não escrito");
}

