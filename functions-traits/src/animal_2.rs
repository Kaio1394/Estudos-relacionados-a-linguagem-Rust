pub mod animais_2{
    // Implementando uma função com o retoro de uma trait através do Box<dyn ...>
    pub struct Lobo{}
    pub struct Vaca{}
    pub trait Animal_2{
        fn fazer_barulho(&self) -> &'static str;
    }
    impl Animal_2 for Lobo{
        fn fazer_barulho(&self) -> &'static str {
            return "Auuuuuuuuu"
        }
    }
    impl Animal_2 for Vaca{
        fn fazer_barulho(&self) -> &'static str {
            return "Muhhhhhhhhh"
        }
    }
    pub fn retorna_animal(numero: f32) -> Box<dyn Animal_2>{
        if numero < 0.0 {
            return Box::new(Lobo{})
        }else{
            return Box::new(Vaca{})
        }
    }
}