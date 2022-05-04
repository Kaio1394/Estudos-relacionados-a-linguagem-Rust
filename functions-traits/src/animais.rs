pub mod animal{
    pub struct Gato{
        pub cor: &'static str
    }
    pub struct Cachorro{
        pub species: &'static str
    }
    pub trait Latido{
        fn latido(&self) -> String;
    }
    pub trait Animal{
        fn fazer_barulho(&self) -> &'static str;
    }

    impl Latido for Cachorro{
        fn latido(&self) -> String {
            return format!("{}", self.species);
        }
    }
    impl Latido for Gato{
        fn latido(&self) -> String {
        return String::from("Miau");
    }
    }
    pub fn latido_teste<T: Latido>(b: T){
        println!("{}", b.latido())
    }

}