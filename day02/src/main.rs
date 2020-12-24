use std::fs;
use std::vec::Vec;

fn main() {
    let filename = "./src/input";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let split_content = content.split("\n").filter(|&x| !x.is_empty());
    let mut items: Vec<String> = Vec::new();

    for s in split_content {
        items.push(s.to_string());
    }

    let mut total_count = 0;

    for item in items {
        // Parse
        let vec = item.split(" ").collect::<Vec<&str>>();

        let condition: Vec<&str> = vec[0].split("-").collect::<Vec<&str>>();
        let unit: Vec<char> = vec[1].chars().collect();
        let text: Vec<char> = vec[2].chars().collect();

        let min = condition[0].parse::<i32>().unwrap();
        let max = condition[1].parse::<i32>().unwrap();

        let mut count = 0;

        for t in text {
            if t == unit[0] {
                count += 1;
            }
        }

        if count >= min && count <= max {
            total_count += 1;
        }
    }

    println!("Total Successful Passwords : {}", total_count);
}
