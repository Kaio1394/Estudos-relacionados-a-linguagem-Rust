pub mod Generic{
    #[derive(Debug)]
    pub struct Point<T>{
        pub x: T,
        pub y: T
    }

    #[derive(Debug)]
    pub struct PointMultiple<T, V>{
        pub x: T,
        pub y: V
    }

    #[derive(Debug)]
    pub struct PointNMultiple<T, V, A>{
        pub x: T,
        pub y: V,
        pub c: A
    }
}