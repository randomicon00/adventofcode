/*Here are the initial and final states of a few more small programs:

1,0,0,0,99 becomes 2,0,0,0,99 (1 + 1 = 2).
2,3,0,3,99 becomes 2,3,0,6,99 (3 * 2 = 6).
2,4,4,5,99,0 becomes 2,4,4,5,99,9801 (99 * 99 = 9801).
1,1,1,4,99,5,6,0,99 becomes 30,1,1,4,2,5,6,0,99.*/

fn day2_solution() {
   println!("Day 2 solution code goes here!");
}


fn main() {
   
    let initial_state = "1,9,10,3,2,3,11,0,99,30,40,50";
   
    let mut curr_state: Vec<u16> = initial_state
                            .split(",")
                            .map(|v| v.parse::<u16>().expect("Not a valid positive number"))
                            .collect();
   

    for i in 0..(curr_state.len()/4) {
       
        if i*(4+1) >= curr_state.len() {
            break;
        }
       
        let idx = i*4;
        println!("idx: {} {}", idx, i);
        match curr_state[i*4] {
            1 => {
                let idx1 = (idx + 1) as usize;
                let idx2 = (idx + 2) as usize;
                let idx3 = (idx + 3) as usize;
                let res = curr_state[curr_state[idx1] as usize] + curr_state[curr_state[idx2] as usize];
                let res_idx = curr_state[idx3] as usize;
                let result = &mut curr_state[res_idx];
                *result = res;
            },
            2 => {
                let idx1 = (idx + 1) as usize;
                let idx2 = (idx + 2) as usize;
                let idx3 = (idx + 3) as usize;
               
                let res = curr_state[curr_state[idx1] as usize] * curr_state[curr_state[idx2] as usize];
                let res_idx = curr_state[idx3] as usize;
                let result = &mut curr_state[res_idx];
                *result = res;
            },
            99 => {
                println!("Found a 99");
                return;
            },
            _ => panic!("Wrong number."),
           
        }
    }
    println!("{:?}", curr_state);
}

#[cfg(test)]
mod tests {
   use super::*;
   
   #[test]
   fn test_true() {
      assert!(true, true);
   }
   #[test]
   fn test_equal() {
      assert_eq!(true, true);
   }
}
