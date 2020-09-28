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
    
    /*
    for i in 0..3 {
        for j in 0..3 {
            match two_dim[i][j] {
                1 => println!("I've go a ONE, with coordinates: {} {}", i, j),
                0 => println!("Not interested by the zero for now..."),
                _ => println!("IMPOSSIBLE"),
            }
        }    
    }*/
    
    let (x, y) = (0, 0);
    
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
}
