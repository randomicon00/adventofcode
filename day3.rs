use std::cmp::min;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: u16,
    y: u16,
}

#[derive(Debug)]
pub struct Line {
    p1: Point,
    p2: Point,
    x: bool,
}

pub fn intersect<'a>(line1: &'a Line, line2: &'a Line) -> bool {
    //x invariant
    if line1.x {
        if (line2.p2.x > line1.p1.x  && line1.p1.x > line2.p1.x) ||
           (line2.p2.x < line1.p1.x  && line1.p1.x < line2.p1.x)
        {
            if (line1.p2.y > line2.p2.y  && line2.p2.y > line1.p1.y) ||
                (line1.p2.y < line2.p2.y  && line2.p2.y < line1.p1.y)
            {
                println!("Intersect 1 is: {}, {}", line1.p1.x, line2.p2.y);        
                return true;
            }
        }
    } else {
        if (line1.p2.x > line2.p1.x  && line2.p1.x > line1.p1.x) ||
           (line1.p2.x < line2.p1.x  && line2.p1.x < line1.p1.x)
        {
            if (line2.p2.y > line1.p2.y  && line1.p2.y > line2.p1.y) ||
                (line2.p2.y < line1.p2.y  && line1.p2.y < line2.p1.y)
            {
                println!("Intersect 2 is: {}, {}", line2.p1.x, line1.p2.y);
                return true;
            }
        }
    }
    return false;
}




fn main() {
   
    let mov1 = [
        Point{ x: 0, y: 0 }, Point{ x: 8, y: 0 }, Point{ x: 8, y: 5 }, Point{ x: 3, y: 5 }, Point{ x: 3, y: 2 },
    ];
    let mov2 = [
        Point{ x: 0, y: 0 }, Point{ x: 0, y: 7 }, Point{ x: 6, y: 7 }, Point{ x: 6, y: 3 }, Point{ x: 2, y: 3 },
    ];
   
    let mut lines_1: Vec<Line> = Vec::new();
    let mut lines_2: Vec<Line> = Vec::new();
   
    for i in 0..(mov1.len()-1)  {
        let mut x = false;
        if mov1[i].x == mov1[i+1].x {
            x = true;
        }
        lines_1.push(Line {
            p1: mov1[i],
            p2: mov1[i+1],
            x,
        });
    }
    //println!("{:?}", lines_1);

    for i in 0..(mov2.len()-1)  {
        let mut x = false;
        if mov2[i].x == mov2[i+1].x {
            x = true;
        }
        lines_2.push(Line {
            p1: mov2[i],
            p2: mov2[i+1],
            x,
        });
    }
    //println!("{:?}", lines_2[1]);
    //included: intersect(&lines_1[3], &lines_2[3]);
    intersect(&lines_1[2], &lines_2[1]);
    //let mut distances: Vec<u16> = Vec::new();
    let mut distance = 10000;
    for i in 0..lines_1.len() {
        //static X
        if lines_1[i].x {
            for line in lines_2.iter().filter(|line| line.x == false) {
                if intersect(&lines_1[i], &line) {
                    //distances.push(lines_1[i].p1.x + line.p2.y)
                    distance = min(lines_1[i].p1.x + line.p2.y, distance);
                }
            }    
        }
        //static y
        else {
            for line in lines_2.iter().filter(|line| line.x == true) {
                if intersect(&lines_1[i], &line) {
                    distance = min(line.p1.x + lines_1[i].p2.y, distance);
                }
            }
        }
    }
    println!("distance: {:?}", distance);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_true() {
        assert_eq!(2, 2);
    }
}


