pub mod first_problem {
    use std::fs::File;
    use std::io::{self, prelude::*, BufReader};
    
    pub fn first_part() -> io::Result<()> {

        let mut file = File::open("./data/1.txt")?;
        let reader = BufReader::new(file);
    
        let mut floor: i32 = 0;
        
        for line in reader.lines() {
            let i = line?;
            let mut split : Vec<char> = i.chars().collect();
            
            for s in split {
                if s == '(' {
                    floor += 1;
                }
                else if s == ')' {
                    floor -= 1;
                }
            }
        }
    
        println!("Day 1 part 1 {}", floor);
        Ok(())
    }
    
    
    pub fn second_part() -> io::Result<()> {
        let mut file = File::open("./data/1.txt")?;
        let reader = BufReader::new(file);
    
        let mut floor: i32 = 0;
        let mut position_where_santa_enters_basement: u32 = 0;
        
        'outer: for line in reader.lines() {
            let i = line?;
            let mut split : Vec<char> = i.chars().collect();
            
            'inner: for s in split {
                if s == '(' {
                    floor += 1;
                }
                else if s == ')' {
                    floor -= 1;
                }
    
                position_where_santa_enters_basement += 1;
    
                if floor < 0 {
                    break 'outer;
                }
            }
        }
    
        println!("Day 1 part 2 {}", position_where_santa_enters_basement);
        Ok(())
    }
}
