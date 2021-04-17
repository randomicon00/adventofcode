fn main() {
  let input_map = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";
  
  println!("This is day 6 solution.");
  for (num, item) in input_map.lines().enumerate() {
    println!("Number: {} Item content: {}", item);
  }
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
