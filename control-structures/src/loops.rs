pub mod loop_test {
    pub fn get_square(limit: i32){
        let mut x = 1;
        while x * x < limit{
            println!("{0} * {0} = {1}", x, x * x);
            x += 1;  
        }   
    }
    
    pub fn get_squareLoop(limit: i32){
        let mut x = 1;
        loop{
            if x * x < limit{
                println!("{0} * {0} = {1}", x, x * x);
                x += 1;
            }else{
                break;
            }
        }
    }
}