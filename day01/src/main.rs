use std::fs;
use std::vec::Vec;

fn main() {
    let filename = "./src/input";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let split_content = content.split("\n");
    let mut items: Vec<i32> = Vec::new();

    for s in split_content {
        if s != "" {
            items.push(s.parse::<i32>().unwrap());
        }
    }

    // Sort the vector ascending
    items.sort();

    for i in 0..items.len() {
        for j in 0..items.len() {
            let diff = 2020 - items[i] - items[j];
            let search = items.binary_search(&diff);

            match search {
                Ok(index) => {
                    println!("{}, {}, {}", items[i], items[j], items[index]);
                    let mul = items[i] * items[j] * items[index];
                    println!("Multiplied : {}", mul);
                    break;
                }
                Err(_error) => {}
            };
        }
    }
}
