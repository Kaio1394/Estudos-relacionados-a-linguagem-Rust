pub mod dev{
    pub struct RustDev{
        pub awesome: bool
    }
    pub struct JavaDev{
        pub awsome: bool
    }
    pub trait Developer{
        fn new(awesome: bool) -> Self;
        fn language(&self) -> &str;
        fn say_hellow(&self){println!("Hellow world");}
    }
    // Implementando o trait Developer aos tructs RustDev e JavaDev
    impl Developer for RustDev{
        
        fn new(awesome: bool) -> Self {
            return RustDev { awesome:  awesome };
        }
        
        fn language(&self) -> &str {
            return "Rust";
        }
        
        fn say_hellow(&self){println!("println!(\"Hellow world\");");}
    }
    impl Developer for JavaDev{
        fn new(awesome: bool) -> Self {
            return JavaDev { awsome: awesome }
        }
        
        fn language(&self) -> &str {
            return "Java"
        }

        fn say_hellow(&self){println!("System.out.println(\"Hellow world\");");}
    }
}