use std::fs;

fn main() {
    let filename = "./src/input";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let split_content = content.split("\n");

    let mut max = 0;
    let mut max_id = 0;
    let mut max_col = 0;
    let mut ids: Vec<i32> = Vec::new();
    for content in split_content {
        let chars = content.chars().collect::<Vec<char>>();
        // let chars = "FBFBBFFRLR".chars().collect::<Vec<char>>();
        // let chars = "BFFFBBFRRR".chars().collect::<Vec<char>>();
        // let chars = "BBFFBBFRLL".chars().collect::<Vec<char>>();
        let mut lower_bound = 0;
        let mut upper_bound = 127;

        let mut col_lower_bound = 0;
        let mut col_upper_bound = 7;
        for c in chars {
            if c == 'F' {
                upper_bound =
                    ((f64::from(lower_bound) + f64::from(upper_bound)) / 2 as f64).floor() as i32;
            } else if c == 'B' {
                lower_bound =
                    ((f64::from(lower_bound) + f64::from(upper_bound)) / 2 as f64).ceil() as i32;
            } else if c == 'L' {
                col_upper_bound = ((f64::from(col_lower_bound) + f64::from(col_upper_bound))
                    / 2 as f64)
                    .floor() as i32;
            } else if c == 'R' {
                col_lower_bound = ((f64::from(col_lower_bound) + f64::from(col_upper_bound))
                    / 2 as f64)
                    .ceil() as i32;
            }

            if lower_bound == upper_bound && col_lower_bound == col_upper_bound {
                let id = (lower_bound * 8) + col_lower_bound;
                if id > max {
                    max = id;
                    max_id = lower_bound;
                    max_col = col_lower_bound;
                }
                ids.push(id);
            }
        }
    }

    println!("Max : {}, Seat id : {}, Column : {}", max, max_id, max_col);

    ids.sort();
    for (i, id) in ids.iter().enumerate() {
        if i == 0 {
            continue;
        }

        if (id - ids[i - 1]) >= 2 {
            println!("i, id : {}, {}", i, id);
            println!("your id : {}", ids[i - 1] + 1);
        }
    }
}
