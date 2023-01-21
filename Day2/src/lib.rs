#[derive(Debug)]
pub enum Choices{
    Rock, 
    Paper, 
    Scissors
}

impl PartialEq for Choices {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Choices::Rock, Choices::Rock) |
            (Choices::Paper, Choices::Paper) |
            (Choices::Scissors, Choices::Scissors) => true,
            _ => false
        }
    }
}
impl Eq for Choices{}

impl Choices{
    fn get_choice_to_win(&self) -> Choices {
        match &self {
            Choices::Rock => Choices::Paper,
            Choices::Paper =>  Choices::Scissors ,
            Choices::Scissors => Choices::Rock,
        }
    }
    fn get_choice_to_lose(&self) -> Choices {
        match &self {
            Choices::Rock => Choices::Scissors,
            Choices::Paper =>  Choices::Rock ,
            Choices::Scissors => Choices::Paper,
        }
    }
    
    pub fn score(input: &Choices) -> u8{
        match input {
            Choices::Rock => 1,
            Choices::Paper => 2,
            Choices::Scissors => 3,
        }
    }
    
    pub fn from_char(input: char) -> Result<Choices,  &'static str> {
        match input {
            'A' | 'X' => Ok(Choices::Rock),
            'B' | 'Y' => Ok(Choices::Paper),
            'C' | 'Z' => Ok(Choices::Scissors),
            _ => Err("asd")
        }
    }

    pub fn should_first_player_win(input: &Choices, second_input: &Choices ) -> Option<bool> {
        let rock = Choices::Rock;
        let paper = Choices::Paper;
        let scissors = Choices::Scissors;
        
        if input == &rock && second_input == &scissors {
            return Some(true)
        }
        else if input == &rock && second_input == &paper {
            return Some(false)
        }
        else if input == &paper && second_input == &rock {
            return Some(true)
        } 
        else if input == &paper && second_input == &scissors{
            return Some(false)
        }
        else if input == &scissors && second_input == &rock {
            return Some(false)
        } 
        else if input == &scissors && second_input == &paper {
            return Some(true)
        }
        else {
            return None
        }  
    }
} 


pub enum Outcome{
    Win, 
    Lose, 
    Draw
}
impl Outcome{
    pub fn from_char(input: char) -> Result<Outcome,  &'static str> {
        match input {
            'X' => Ok(Outcome::Lose),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err("asd")
        }
    }
    pub fn score(input: &Outcome) -> u8{
        match input {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        }
    }
}

pub fn get_expected_choice(input: Choices, expected_outcome: Outcome ) -> Choices{
    match expected_outcome {
        Outcome::Win => input.get_choice_to_win(),
        Outcome::Lose => input.get_choice_to_lose(),
        Outcome::Draw => input,
    }
}
