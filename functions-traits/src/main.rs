#[allow(unused_imports)]

// Macro
// Em macros podemos ter tipos de parâmetro: 
// expr, ident, block, stmt, pat, path, meta, ty e tt
macro_rules! name {
    ($name: expr) => (
        println!("My name is {}", $name);
    );
}
// macro com vários parâmetros
macro_rules! name {
    ($($name: expr), *) => { 
        $(println!("Hey {}", $name);)*
    };
}
// macro com variáveis
macro_rules! xy {
    (x => $e: expr) => {println!("X is {}", $e);};
    (y => $e: expr) => {println!("Y is {}", $e);};
}
// macro que constroi uma função
macro_rules! build_fn {
    ($fn_name: ident) => {
        fn $fn_name(){
            println!("{:?} this function was called", stringify!($fn_name));
        }
    };
}

struct RustDev{
    awesome: bool
}
struct JavaDev{
    awsome: bool
}
pub trait Developer{
    fn new(awesome: bool) -> Self;
    fn language(&self) -> &str;
    fn say_hellow(&self){println!("Hellow world");}
}
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
fn main() {
    // Usando o macro
    name!("Kaio");
    xy!(x => 5);
    xy!(y => "Kaio");
    build_fn!(hey);
    hey();
    let java_dev = JavaDev{
        awsome: true
    };
    println!("{}", java_dev.language());
    java_dev.say_hellow();
}
