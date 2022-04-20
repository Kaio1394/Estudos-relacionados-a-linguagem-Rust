#[derive(Debug)]
pub enum Colors{
    Red,
    Black,
    White
}

#[derive(Debug)]
pub enum ColorsGeneric<T>{
    Red(T),
    Black(T),
    White(T)
}