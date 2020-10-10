/*
It is a six-digit number.
The value is within the range given in your puzzle input.
Two adjacent digits are the same (like 22 in 122345).
Going from left to right, the digits never decrease; 
they only ever increase or stay the same (like 111123 or 135679).
*/

// Puzzle input is 125730-579381

fn valid_number(num: u32) -> bool {
    let splitted = format!("{}", num);
    
    let initial = 0;
    let digits: Vec<u32> = splitted.chars()
                    .map(|x| x.to_string().parse::<u32>().unwrap())
                    .collect();
    println!("{:?}", digits);
    true
}

fn main() {
    let puzzle_input: Vec<u32> = "125730-579381"
            .split("-")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
            
    valid_number(puzzle_input[0]);
}
