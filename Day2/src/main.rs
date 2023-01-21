use std::fs;

use Day2::{Choices,Outcome,get_expected_choice};

fn main() {
    let input = fs::read_to_string("../artifacts/Day02/input.txt")
        .expect("Should have been able to read the file");

    let mut result: u32 = 0;

    for line in input.lines(){
        let mut chars = line.chars();

        let opponent = Choices::from_char(chars.nth(0).unwrap()).unwrap();
        let input = Choices::from_char(chars.nth(1).unwrap()).unwrap();

        match Choices::should_first_player_win(&input, &opponent) {
        
            Some(outcome) => {
                if outcome{
                    result += 6 + Choices::score(&input) as u32
                }
                else{
                    result += Choices::score(&input)as u32
                }
            }
            None => result += 3 + Choices::score(&input)as u32
        } 
        
    }
    println!("part 1 {}", result);

    result = 0;
    //part 2
    for line in input.lines(){
        let mut chars = line.chars();

        let opponent = Choices::from_char(chars.nth(0).unwrap()).unwrap();
        let outcome =  Outcome::from_char(chars.nth(1).unwrap()).unwrap();
        // let expected = get_expected_choice(opponent, outcome) ;
        
        let a = Outcome::score(&outcome);

        let t = match get_expected_choice(opponent, outcome) {
            Choices::Rock => Choices::score(&Choices::Rock),
            Choices::Paper => Choices::score(&Choices::Paper),
            Choices::Scissors => Choices::score(&Choices::Scissors),
        };
        result += t as u32 + a as u32 ;   
    }
    println!("part 2 {}", result);


}