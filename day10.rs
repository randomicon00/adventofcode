pub fn compute_visible(a_p: &[(usize, usize)]) -> usize {
    0
}

//Algorithm basis is ready
fn main() {
    const SIZE: usize = 5;
    let asteroids_pos = "
.#..#
.....
#####
....#
...##
";
    
    const SHARP: &str = "#";
    const POINT: &str = ".";
    let mut offset = 0;
    let mut asteroids_arr: [[&str; SIZE]; SIZE] = [[""; SIZE]; SIZE];
    let mut asteroids: Vec<(usize, usize)> = Vec::new(); 
    
    for (index, c) in asteroids_pos.chars().enumerate() {
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
    //println!("Here is the array: {:?}", asteroids_arr)
    
    
    //Algorithm 
    
    //Loop over all asteroids (SHARP).
    //Check all diagonals
    //Add one and one  (bottom right)
    //Add one and remove one (bottom left)
    //Add one and remove one (top right)
    //Remove one and add one (top left)
    //Include 'Add 0.5 and 2' and so on.
    
    /*for i in asteroids_arr.iter() {
        println!("asteroids are: {:?}", i);
    }
    for asteroid in asteroids {
        println!("asteroids location {:?}", asteroid);
    }*/
    
    let two_dim = [[0, 1, 1], [0, 1, 1], [0, 1, 1]];
    println!("Single elem is {}", &two_dim[2][0]);
    
   
    let (x, y) = (2, 0);
    
    let start = x+1;  //start + 1
    let end = two_dim.len();
    //Vertical down - x value is fixed
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
    
    //Vertical up TODO
    let two_dim2 = [[0, 1, 1, 1, 1, 1, 1, 1], [0, 1, 1, 1, 1, 1, 1, 1], [0, 1, 1, 1, 1, 1, 1, 1], [0, 1, 1, 1, 1, 1, 1, 1], 
        [0, 1, 1, 1, 1, 1, 1, 1], [0, 1, 1, 1, 1, 1, 1, 1], [0, 1, 1, 1, 1, 1, 1, 1], [0, 1, 1, 1, 1, 1, 1, 1]];
    let (x2, y2) = (0, 0);
    let (start2, end2) = (0, two_dim2.len());
    let mut obstacle2 = false;
    for i in (0..y2).rev() {
        println!("I AM HERE ")
    }
    
    
    
    
    //Diagonal bottom right
    println!("---------------------------------------");
    for x_pos in (x2+1)..end2 {
        //println!("Bottom right: ({},{})", x_pos, y2 + (x_pos - x2));
        println!("");
    }
    println!("---------------------------------------");
    //diagonal bottom left
    for x_pos in (0..x2).rev() {
        //println!("Bottom left: ({},{})", x_pos, y2 + (x2 - x_pos));
        println!("");
    }
    
    let (x3, y3) = (1, 2);
    //diagonal top left 
    for (index, x_pos) in (0..(x3+1)).rev().enumerate() {
        //println!("Top left: ({},{})", x_pos, y3 - index);
        println!("");
    }
    //diagonal top right  
    for (index, x_pos) in (x3+1..end2).enumerate() {
        //println!("Top right: ({},{})", x_pos, y3 - index-1);
        println!("");
    }
    
    
    //Each diagnoal, first compute the distance between the point and 
    //the end For instance
    // X = 2 and end = 4
    // X should move 2 + 2 (minimum) equals 6
    // if (x_pos + increment*2) > end -> return 
    // if (x_position - increment*2) < 0 -> return 
    let length = two_dim2.len();
    
    println!("Here are the possibilities {}", length);
    println!("Here are the possibilities {}, divide by 2 and substract 1", (length / 2) - 1);
    println!("which gives the following possiiblities: from 2 to {}", (length / 2) - 1)
    
    

    
}
