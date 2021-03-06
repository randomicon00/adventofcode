/*
Advent of code day 5

ABCDE
 1002

 DE - two-digit opcode,      02 == opcode 2
 C - mode of 1st parameter,  0 == position mode
 B - mode of 2nd parameter,  1 == immediate mode
 A - mode of 3rd parameter,  0 == position mode,
 omitted due to being a leading zero
*/

fn main() {
  let input = "DE - two-digit opcode,      02 == opcode 2
 C - mode of 1st parameter,  0 == position mode
 B - mode of 2nd parameter,  1 == immediate mode
 A - mode of 3rd parameter,  0 == position mode";
  println!("Greetings!. Here is the input content for the problem of Advent of code day 5: 
    {}", input);
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_true() {
        assert!(true);
    }
  
    #[test]
    fn test_equal() {
        assert_eq!(true, true);
    }
}
