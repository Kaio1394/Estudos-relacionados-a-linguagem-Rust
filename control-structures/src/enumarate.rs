pub mod enum_teste{
    #[derive(Debug)]
    pub enum Suit{
        Heart,
        Spade,
        Club,
        Diamond
    }

    // Switch case em rust
    pub fn print_choice(choice: Suit){
        match choice{
            Heart => {println!("\u{2665}")}
            Spade => {println!("\u{2668}")}
            Club => {println!("\u{2663}")}
            Diamond => {println!("\u{2666}")}

        }
    }

    pub fn country(code: i32) -> String{
        return match code {
            44 => String::from("UK"),
            34 => String::from("Spain"),
            35..=99 => String::from("Unknown"),
            _ => String::from("Invalid")
        };
    }

    pub fn get_oranges(amount: i32) -> &'static str{
        return match amount {
            0 => "no",
            1 | 2 => "one or two",
            3..=7 => "a few",
            _ if (amount % 2 == 0) => "an even amout of",
            _ => "lots of"
        };
    }
}