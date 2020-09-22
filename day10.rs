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
                asteroids_arr[(index-offset)/5 as usize][index-offset - ((index-offset)/5 as usize)*5] = SHARP;
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
    
    //Check all diagonals
    //Add one and one  (bottom right)
    //Add one and remove one (bottom left)
    //Add one and remove one (top right)
    //Remove one and add one (top left)
    
    /*for i in asteroids_arr.iter() {
        println!("asteroids are: {:?}", i);
    }*/
    
    for asteroid in asteroids {
        println!("asteroids location {:?}", asteroid)
    }
}
