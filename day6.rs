fn main() {
  let hello_world = "hello world".to_string();
  println!("{}", hello_world);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_true() {
        assert_eq!(2, 2);
    }
}
