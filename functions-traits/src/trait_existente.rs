pub mod trait_existente{
    use std::ops::Add;

    #[derive(Debug)]
    pub struct Ponteiro{
        pub X: f32,
        pub Y: f32
    }

    // Implementando o trait Add em um estrutura qualquer
    impl Add for Ponteiro{
        type Output = Ponteiro;

        fn add(self, rhs: Self) -> Self::Output {
            return Ponteiro{
               X: self.X + rhs.X,
               Y: self.Y + rhs.Y
            }
        }
    }
}