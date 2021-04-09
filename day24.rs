/*
What is the biodiversity rating for the first layout that appears twice?
*/

fn main() {
  let input = "What is the biodiversity rating for the first layout that appears twice?";
  println!("Hello world day 24 of the advent of code
     {}", input);
  
  println!("The answer will be available here.");
  println!("Input data needs to be added and displayed here.");
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
      assert_eq!("one", "one");
    }
    
    #[test]
    fn test_display_input() {
      assert_eq!("input", "input");
    }
}
