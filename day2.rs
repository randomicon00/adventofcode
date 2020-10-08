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
    #[test]
    fn test() {
        assert_eq!(true, true);
    }
}

