/*
<x=16, y=-11, z=2>
<x=0, y=-4, z=7>
<x=6, y=4, z=-10>
<x=-3, y=-2, z=-4>
*/

fn main() {
  
  let input = "<x=16, y=-11, z=2>
<x=0, y=-4, z=7>
<x=6, y=4, z=-10>
<x=-3, y=-2, z=-4>"
  
  println!("Advent of code day 11 input is: 
    {}", input);
}

#[cfg(test)]
mod tests {
  
  #[test]
  fn test_true() {
    assert!(true);
  }
  #[test]
  fn test_equal() {
    assert_eq!("equal", "equal");
  }
}
