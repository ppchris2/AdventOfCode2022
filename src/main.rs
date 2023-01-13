use std::fs;

fn main() {

    // let string = "1000
    // 2000
    // 3000
    
    // 4000
    
    // 5000
    // 6000
    
    // 7000
    // 8000
    // 9000
    
    // 10000";

    let contents = fs::read_to_string("./artifacts/Day01/input.txt")
        .expect("Should have been able to read the file");


    let lines = contents.lines();
    let mut vec = Vec::new();
    let mut count = 0;

    for item in lines {

        
        let trimmed = item.trim().to_string();
        if trimmed == ""  {
            vec.push(count);
            count = 0;
        }
        
        else{
            let num : u32 = trimmed.parse().expect("What");
            count = num + count
        }
    }
    vec.push(count);
    vec.sort();

    let answer1 = vec.last().unwrap();
    println!("answer1: {}", answer1);
    let sum2 = &vec[vec.len()-3..];
    
    let sum: u32 = sum2.iter().sum();

    println!("answer2: {}", sum)
}
