fn main() {   
  let input = "cut -7812
deal with increment 55
cut -3909
deal with increment 51
deal into new stack
deal with increment 4
cut -77
deal with increment 26
deal into new stack
deal with increment 36
cut 5266
deal with increment 20
cut 8726
deal with increment 22
cut 4380
deal into new stack
cut 3342
deal with increment 16
cut -2237
deal into new stack
deal with increment 20
cut 7066
deal with increment 18
cut 5979
deal with increment 9
cut 2219
deal with increment 44
cut 7341
deal with increment 10
cut -6719
deal with increment 42
deal into new stack
cut -2135
deal with increment 75
cut 5967
deal into new stack
cut 6401
deal with increment 39
deal into new stack
deal with increment 56
cut 7735
deal with increment 49
cut -6350
deal with increment 50
deal into new stack
deal with increment 72
deal into new stack
cut 776
deal into new stack
deal with increment 18
cut 9619
deal with increment 9
deal into new stack
cut 5343
deal into new stack
cut 9562
deal with increment 65
cut 4499
deal with increment 58
cut -4850
deal into new stack
cut -9417
deal into new stack
deal with increment 33
cut 2763
deal with increment 61
cut 7377
deal with increment 27
cut 895
deal into new stack
deal with increment 41
cut -1207
deal with increment 22
cut -7401
deal with increment 48
cut 5776
deal with increment 3
cut 2097
deal with increment 49
cut -8098
deal with increment 68
cut 2296
deal with increment 35
cut -4471
deal with increment 56
cut -2778
deal with increment 5
cut -6386
deal with increment 54
cut -7411
deal with increment 20
cut -4222
deal into new stack
cut -5236
deal with increment 64
cut -3581
deal with increment 11
cut 3255
deal with increment 20
cut -5914";
  
  println!("Welcome advent of code day 22");
  
  for (i, line) in input.lines().enumerate() {
    println!("line: {}, content: {}", i, line);
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
