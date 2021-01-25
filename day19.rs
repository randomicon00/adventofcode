fn main() {
  let input = r"       X
  0->      9
 0#.........
 |.#........
 v..##......
  ...###....
  ....###...
Y .....####.
  ......####
  ......####
  .......###
 9........##
 ";
  
  println!("Advent of code day 19:
      {}", input);
}

#[cfg(test)]
mod tests {
    
    #[test]
    fn test_true() {
        assert!(true);
    }
    #[test]
    fn test_equality() {
        assert_eq!(true, true);
    }
}
