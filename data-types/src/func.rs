pub mod update{
    pub fn update_colors(colors_slice: &mut [&str]){
        colors_slice[0] = "yellow";
        colors_slice[1] = "black";
    }
    
    pub fn change_string(string: &mut &str){
        *string = "Eliziane";   
    }
}