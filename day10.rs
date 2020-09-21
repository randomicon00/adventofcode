//Populated the array
fn main() {
    const SIZE: usize = 5;
    let asteroids = "
.#..#
.....
#####
....#
...##
";
    
    const SHARP: &str = "#";
    const POINT: &str = ".";
    let mut offset = 0;
    let mut array: [[&str; SIZE]; SIZE] = [[""; SIZE]; SIZE];

    for (index, c) in asteroids.chars().enumerate() {
        let c = &(c.to_string()[..]);
        match c {
            SHARP => {
                array[(index-offset)/5 as usize][index-offset - ((index-offset)/5 as usize)*5] = SHARP;
            },
            POINT => {
                array[(index-offset)/5 as usize][index-offset - ((index-offset)/5 as usize)*5] = POINT;
            },
            _ => {
                offset += 1;    
            }
        }
    }
    println!("Here is the array: {:?}", array)
}
