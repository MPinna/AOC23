use std::fs;
use std::collections::HashMap;


static INPUT_FILENAME: &str = "input";


fn get_calibration(line: &str) -> u32 {

    let digits: Vec<u32> = line.chars()
        .filter_map(|d| d.to_digit(10))
        .collect();
        let f: Option<&u32> = digits.first();
        let l: Option<&u32> = digits.last();

        if f.is_some() && l.is_some() {
            let d: &u32 = f.expect("First number not found");
            let u: &u32 = l.expect("Last number not found");

            return d*10 + u;
        }
        else {
            return 0;
        }
}

fn get_calibration2(line: &str) -> u32 {

    // println!("{}", line);
    let numbers: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut d: Option<u32> = None;
    let mut u: Option<u32> = None;

    for i in 1..(line.len() + 1){
        if d.is_none(){
            let head: &str = &line[..i];
            let head_char: char = head.chars().nth(i - 1).unwrap();
            if head_char.is_digit(10){
                d = head_char.to_digit(10);
            }
            else {
                for k in numbers.keys(){
                    if head.contains(k) {
                        d = Some(numbers[k]);
                        break;
                    }
                }
            }
        }

        if u.is_none(){
            let tail: &str = &line[(line.len() - i)..];

            let tail_char: char = tail.chars().next().unwrap();
            if tail_char.is_digit(10){
                u = tail_char.to_digit(10);
            }

            for k in numbers.keys(){
                if tail.contains(k) {
                    u = Some(numbers[k]);
                    break;
                }
            }
        }
    }

    assert!(d.is_some() && u.is_some(), "Program panics with the following string: {}", line);

    if d.is_some() && u.is_some() {
        // println!("{} {}", d.unwrap(), u.unwrap());
        return d.unwrap()*10 + u.unwrap();
    }
    else {
        return 0;
    }
}

fn main() {
    let contents: String = fs::read_to_string(INPUT_FILENAME)
        .expect("Should have been able to read the input file");

    let lines: std::str::Lines<'_> = contents.lines();

    let mut sum: u32 = 0;

    for line in lines {

        // part 1
        // let calibration: u32 = get_calibration(line);
        
        let calibration: u32 = get_calibration2(line);
        sum += calibration;
    }
    
    println!("The total sum is: {}", sum);
}
