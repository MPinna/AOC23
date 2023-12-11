use std::fs;
use std::collections::HashMap;

static TEST_FILENAME: &str = "test.txt";
static INPUT_FILENAME: &str = "input";

fn is_game_possible(game_line: &str) -> bool {

    let cubes_map: HashMap<&str, u32> = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);

    let sets: std::str::Split<'_, &str> = game_line.split(";");

    for set in sets {
        let groups: std::str::Split<'_, &str> = set.split(",");

        for group in groups {
            let mut splits = group.trim_start()
                                                    .split(" ");
            let num: u32 = splits.next().unwrap()
                                .parse().unwrap();

            let color: &str = splits.next().unwrap()
                                .trim();

            if num > cubes_map[color] {
                return false;
            }
        }
    }

    return true;
}

fn get_cube_set_power(game_line: &str) -> u32 {
    let sets: std::str::Split<'_, &str> = game_line.split(";");

    let mut cubes_map: HashMap<&str, u32> = HashMap::from(
        [
            ("red", 0),
            ("green", 0),
            ("blue", 0)
        ]
    );

    for set in sets {
        let groups: std::str::Split<'_, &str> = set.split(",");

        for group in groups {
            let mut splits = group.trim_start()
                                                    .split(" ");
            let num: u32 = splits.next().unwrap()
                                .parse().unwrap();

            let color: &str = splits.next().unwrap()
                                .trim();

                                    
            cubes_map.entry(color).and_modify(|v| *v = std::cmp::max(*v, num));
        }
    }

    return cubes_map.values().product()
}

fn main() {
    let contents: String = fs::read_to_string(INPUT_FILENAME)
        .expect("Should have been able to read the input file");

    let lines: std::str::Lines<'_> = contents.lines();

    let mut sum: i32 = 0;
    let mut prod: u32 = 0;

    for line in lines {
        let mut splits: std::str::Split<'_, &str> = line.split(":");
        let game_index: i32 = splits.next().unwrap()
                                .split(" ")
                                .last().unwrap()
                                .parse().unwrap();
        let game_body: &str = splits.next().unwrap();

        // println!("Game index is: {}", game_index);

        if is_game_possible(game_body) {
            sum +=  game_index;
        }

        prod += get_cube_set_power(game_body);
    }
    
    println!("The total sum is: {}", sum);
    println!("The sum of the power of the sets is: {}", prod);
}