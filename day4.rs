/*
It is a six-digit number.
The value is within the range given in your puzzle input.
Two adjacent digits are the same (like 22 in 122345).
Going from left to right, the digits never decrease; 
they only ever increase or stay the same (like 111123 or 135679).
*/

fn valid_number(num: u32) -> bool {
    let splitted = format!("{}", num);
    
    let digits: Vec<u32> = splitted.chars()
                    .map(|x| x.to_string().parse::<u32>().unwrap())
                    .collect();
    
    let mut valid = true;
    for i in 0..(digits.len()-1) {
        if digits[i] <= digits[i+1] {
            //valid
        } else {
            valid = false;
            println!("mismatch value: {} <-> {}", digits[i], digits[i+1])
            break;
        }
    }
    
    println!("{:?}", digits);
    valid
}

fn main() {
    let puzzle_input: Vec<u32> = "125730-579381"
            .split("-")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
            
    println!("{}", valid_number(125788));
}

//Tests for the code problem number 4
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_function() {
        assert!(true)
    }
}
