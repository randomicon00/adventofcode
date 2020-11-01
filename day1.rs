//Solution for the advent of code problem number 1
//For example:
//For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
//For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
//For a mass of 1969, the fuel required is 654.
//For a mass of 100756, the fuel required is 33583.

pub fn fuel_required(mass: usize) -> usize {
  (mass/3) - 2
}

//Tests for the code problem number 1
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fuel_required_input1() {
        assert_eq!(2, fuel_required(12))
    }
    #[test]
    fn test_fuel_required_input2() {
        assert_eq!(2, fuel_required(14))
    }
    #[test]
    fn test_fuel_required_input3() {
        assert_eq!(654, fuel_required(1969))
    }
    #[test]
    fn test_fuel_required_input4() {
        assert_eq!(33583, fuel_required(100756))
    }
}

