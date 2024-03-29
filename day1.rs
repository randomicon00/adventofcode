/// Advent of code problem's 1 solution.
/// Rest of solutions will follow.
/// Problem's scenario:
/// For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
/// For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
/// For a mass of 1969, the fuel required is 654.
/// For a mass of 100756, the fuel required is 33583.

/// Compute the quantity of fuel required.
fn fuel_required(mass: usize) -> usize {
  mass / 3 - 2
}

/// Entry point of the program
fn main() {
  let mass = 12;
  
  println!("Fuel required when mass is {}, is: {}", mass, fuel_required(mass));
  
  // More examples
  for mass_item in [14, 30, 1969, 100756] {
    println!("Fuel required when mass is {}, is {}", mass_item , fuel_required(mass_item));  
  }                        
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_required_input_one() {
        assert_eq!(2, fuel_required(12));
    }
    #[test]
    fn test_fuel_required_input_two() {
        assert_eq!(2, fuel_required(14));
    }
    #[test]
    fn test_fuel_required_input_three() {
        assert_eq!(654, fuel_required(1969));
    }
    #[test]
    fn test_fuel_required_input_four() {
        assert_eq!(33583, fuel_required(100756));
    }
}
