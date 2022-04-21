pub mod condic {
    pub fn verifica_numero(numero: i64) -> String{
        if numero > 5 {
            return format!("Numero {} é maior que 5", numero);
        }else if numero == 5{
            return format!("Número {} é igual que 5", numero);
        }else{
            return format!("Número {} é menor que 5", numero);
        }
    }
    pub fn verifica_numero_menos_linha_codigo(numero: i64) -> String{
        return if numero >= 5 {format!("{} >= 5", numero)} else {format!("{} =< 5", numero)};
    }
}