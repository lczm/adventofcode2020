use std::convert::TryFrom;
use std::fs;
use std::vec::Vec;

struct Position {
    x: u32,
    y: u32,
}

fn main() {
    let filename = "./src/input";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let split_content = content.split("\n").filter(|&x| !x.is_empty());
    let mut items: Vec<String> = Vec::new();
    for s in split_content {
        items.push(s.to_string());
    }

    let mut map: Vec<Vec<char>> = Vec::new();
    for item in items {
        map.push(item.chars().collect());
    }

    // {0, 0} is top-left
    // moving downwards is going positive

    let move_right = 3;
    let move_down = 1;

    let width = map[0].len() as u32;
    let height = (map.len() - 1) as u32;

    let mut current_position = Position { x: 0, y: 0 };
    let mut trees_found = 0;

    while current_position.y != height {
        current_position.x += move_right;
        current_position.y += move_down;

        let mod_x = usize::try_from(current_position.x % width).unwrap();
        let mod_y = usize::try_from(current_position.y).unwrap();

        if map[mod_y][mod_x] == '#' {
            trees_found += 1;
        }
    }

    println!("Trees : {}", trees_found);
}
