pub mod job{
    // #[derive(Debug)] -> serve para printar o struct
    #[derive(Debug)]
    pub struct Employee{
        pub name: String,
        pub company: String,
        pub age: u32
    }

    impl Employee{
        pub fn fn_details(&self) -> String{
            return format!("name: {}, company: {}, age:{},", 
                self.name, self.company, self.age)
        }
        // Método estático
        pub fn static_fn_detail() -> String{
            return  String::from("Detalhes...");
        }
    }
}