#[allow(dead_code)]
pub mod arquivo{
    use std::fs::rename;
use std::io::{Read, self};
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    pub fn criacao_arquivo_rs(path: &'static str) -> File{
        if path.is_empty() && !path.contains(".rs"){
            return File::create("src/example.rs").expect("Criação do arquivo falhou");
        }
        else{
            return File::create(path).expect("Criação do arquivo falhou");
        }
    }
    
    pub fn criacao_codigo_rust(codigo: &'static str, mut file: File){
        file.write_all(codigo.as_bytes()).expect("Código não escrito");
    }

    pub fn criacao_codigo_txt(texto: &'static str, mut file: File){
        file.write_all(texto.as_bytes()).expect("Código não escrito");
    }

    pub fn criacao_arquivo_txt(path: &'static str) -> File{
        if path.is_empty() && !path.contains(".txt"){
            return File::create("src/example.txt").expect("Criação do arquivo falhou");
        }
        else{
            return File::create(path).expect("Criação do arquivo falhou");
        }
    }
    pub fn let_arquivo(path: &'static str) -> String{
        let path_arquivo = Path::new(path);

        if !Path::exists(path_arquivo){
            panic!("Path inválido!!!");   
        }
        else{
            let mut string_lida_arquivo = String::from(path);
            let mut file = File::open(path).unwrap();
            file.read_to_string(&mut string_lida_arquivo).unwrap();
            return string_lida_arquivo;
        }     
    }

    pub fn renomeando_arquivo(path_atual: &'static str, path_novo: &'static str){
        let path1 = Path::new(path_atual);
        let path2 = Path::new(path_novo);
        if !Path::exists(&path1) && !Path::exists(&path2){
            panic!("Path inválido!!!");      
        }else {
            rename(path_atual, path_novo).expect("Arquivo não encontrado");
        }
    }

}