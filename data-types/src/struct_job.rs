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
            return format!("name: {}, age:{}, company: {}", 
                self.name, self.company, self.age)
        }
    }
}