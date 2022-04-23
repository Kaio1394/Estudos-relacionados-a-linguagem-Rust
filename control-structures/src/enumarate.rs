pub mod enum_teste{
    #[derive(Debug)]
    pub enum Suit{
        Heart,
        Spade,
        Club,
        Diamond
    }

    pub fn print_choice(choice: Suit){
        match choice{
            Heart => {println!("\u{2665}")}
            Spade => {println!("\u{2668}")}
            Club => {println!("\u{2663}")}
            Diamond => {println!("\u{2666}")}

        }
    }
}