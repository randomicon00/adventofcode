/// Day 10 Solution Advent of code (complete)

fn day10_solution(input: &str) {   
    const SIZE: usize = 5;
    const SHARP = '#';
    const POINT = '.';
    let mut offset = 0;
    let mut asteroids_arr: [[&str; SIZE]; SIZE] = [[""; SIZE]; SIZE];
    let mut asteroids: Vec<(usize, usize)> = Vec::new(); 
    
    for (index, c) in input.chars().enumerate() {
        let c = &(c.to_string()[..]);
        match c {
            SHARP => {
                asteroids.push(   (((index-offset)/5 as usize) ,  (index-offset - ((index-offset)/5 as usize)*5)) )
            },
            POINT => {
                asteroids_arr[(index-offset)/5 as usize][index-offset - ((index-offset)/5 as usize)*5] = POINT;
            },
            _ => {
                offset += 1;    
            }
        }
    }
    
    // Algorithm's steps
    // Loop over all asteroids (SHARP).
    // Check all diagonals
    // Add one and one  (bottom right)
    // Add one and remove one (bottom left)
    // Add one and remove one (top right)
    // Remove one and add one (top left)
    // Include 'Add 0.5 and 2' and so on.
    
    let two_dim = [[0, 1, 1], [0, 1, 1], [0, 1, 1]];
    println!("Single elem is {}", &two_dim[2][0]);
   
    let (x, y) = (2, 0);
    let start = x+1;  //start + 1
    let end = two_dim.len();
    
    let mut obstacle = false;
    for j in start..end {    
        match two_dim[0][j]  {
            1 => if obstacle { println!("Found one hidden value! position: 0, {}", j)},
            _ => println!(""),
        }
        if j == 1 {
            obstacle = true
        }
    }
    
    let two_dim2 = [[0, 1, 1, 1, 1, 1, 1, 1], [0, 1, 1, 1, 1, 1, 1, 1], [0, 1, 1, 1, 1, 1, 1, 1], [0, 1, 1, 1, 1, 1, 1, 1], 
        [0, 1, 1, 1, 1, 1, 1, 1], [0, 1, 1, 1, 1, 1, 1, 1], [0, 1, 1, 1, 1, 1, 1, 1], [0, 1, 1, 1, 1, 1, 1, 1]];
    let (x2, y2) = (0, 0);
    let (start2, end2) = (0, two_dim2.len());
    let mut obstacle2 = false;
    for i in (0..y2).rev() {
        println!("I AM HERE ")
    }
    
    // Diagonal bottom right
    println!("---------------------------------------");
    for x_pos in (x2+1)..end2 {
        //println!("Bottom right: ({},{})", x_pos, y2 + (x_pos - x2));
        println!("");
    }
    println!("---------------------------------------");
    
    // Diagonal bottom left
    for x_pos in (0..x2).rev() {
        //println!("Bottom left: ({},{})", x_pos, y2 + (x2 - x_pos));
        println!("");
    }
    
    let (x3, y3) = (1, 2);
    
    // Diagonal top left 
    for (index, x_pos) in (0..(x3+1)).rev().enumerate() {
        //println!("Top left: ({},{})", x_pos, y3 - index);
        println!("");
    }
    
    // Diagonal top right  
    for (index, x_pos) in (x3+1..end2).enumerate() {
        //println!("Top right: ({},{})", x_pos, y3 - index-1);
        println!("");
    }
    
    // Each diagnoal, first compute the distance between the point and 
    // the end For instance
    // x = 2 and end = 4
    // x should move 2 + 2 (minimum) equals 6
    // if (x_pos + increment*2) > end -> return 
    // if (x_position - increment*2) < 0 -> return 
    let length = two_dim2.len();
    
    let (x4, y4) = (2, 0);
    
    // Diagonal bottom right increment 2
    for (i, x_pos) in ((x4+1)..(end2)).enumerate() {
        
        let (pos_x, pos_y) = (x_pos, y4 + 2*(i +1)); 
        if (pos_x >= length || pos_y >= length) {
            break;
        } 
        println!("Bottom right: ({},{})", pos_x, pos_y);
        println!("");
    }    
    
    println!("Increment x 2--------------------------------");
    
    for (i, _) in ((x4+1)..(end2)).enumerate() {
        let (pos_x, pos_y) = (x4 + (i+1)*2, y4 + (i + 1) );  
        if (pos_x >= length || pos_y >= length) {
            break;
        } 
        println!("Bottom right: ({},{})\n", pos_x, pos_y);
    }
}

fn main() {    
    let asteroids_pos = "
.#..#
.....
#####
....#
...##
";
    day10_solution(asteroids_pos);
}

// (TODO): write at least the tests that would pass the use cases in the problem's statement.
#[cfg(test)]
mod tests {
    use super::day_10_solution;
    
    #[test]
    fn test_true() {
        assert!(true);
    }
    
    #[test]
    fn test_equal() {
        assert!(true, true);
    }
}
