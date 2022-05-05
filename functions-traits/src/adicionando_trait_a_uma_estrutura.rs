pub mod adicionando_trait{
    pub trait Somatorio<T>{
        fn soma(&self) -> T;
    }
    impl Somatorio<i32> for Vec<i32>{
        fn soma(&self) -> i32 {
            let mut sum: i32 = 0;
            for i in self{
                sum += *i;
            }
            return sum;
        }
    }
}