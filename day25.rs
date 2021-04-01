/*
Santa's ship is a Reindeer-class starship; these ships use pressure-sensitive floors to 
determine the identity of droids and crew members. The standard configuration for these 
starships is for all droids to weigh exactly the same amount to make them easier to detect. 
If you need to get past such a sensor, you might be able to reach the correct weight by 
carrying items from the environment.

Look around the ship and see if you can find the password for the main airlock.
*/
fn main() {
  let input = "data1
  data2
  data3";
  
  for (num, line) of input.lines().enumerate() {
    println!("Day 25, line {}, content: {}", num, line);
    println!("----------------------------------------");
  }
}
#[cfg(test)]
mod tests {
    use super::*;
     
    #[test]
    fn test_true() {
        assert_eq!(true);
    }
  
    #[test]
    fn test_equal() {
        assert_eq!(true, true);
    }
}
