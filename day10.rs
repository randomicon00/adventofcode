
//First commit
fn main() {
    const SIZE: usize = 5;
    let asteroids = "
.#..#
.....
#####
....#
...##
";
    const DASH: &str = "#";
    const POINT: &str = ".";
    let mut offset = 0;
    for (index, c) in asteroids.chars().enumerate() {
        let c = &(c.to_string()[..]);

        match c {
            DASH => println!("this is a dash {} x:{} y:{}", index-offset, (index-offset)/5 as usize,  index-offset - ((index-offset)/5 as usize)*5),
            POINT => println!("this is a point {} x:{} y:{}", index-offset, (index-offset)/5 as usize, index-offset - ((index-offset)/5 as usize)*5),
            _ => {
                println!("not interested...");
                offset += 1;    
            }
        }
    }
    let array: [[&str; 5]; 5] = [[""; 5]; 5];
}
