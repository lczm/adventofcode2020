use std::vec::Vec;
use std::{fs, unimplemented};

struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String,
}

impl Passport {
    fn new() -> Passport {
        Passport {
            byr: String::new(),
            iyr: String::new(),
            eyr: String::new(),
            hgt: String::new(),
            hcl: String::new(),
            ecl: String::new(),
            pid: String::new(),
            cid: String::new(),
        }
    }
}

fn update_passport(key: &str, item: &String, passport: &mut Passport) {
    if item.contains(key) {
        if key == "byr" {
            passport.byr = slice_item(key, item);
        }
        if key == "iyr" {
            passport.iyr = slice_item(key, item);
        }
        if key == "eyr" {
            passport.eyr = slice_item(key, item);
        }
        if key == "hgt" {
            passport.hgt = slice_item(key, item);
        }
        if key == "hcl" {
            passport.hcl = slice_item(key, item);
        }
        if key == "ecl" {
            passport.ecl = slice_item(key, item);
        }
        if key == "pid" {
            passport.pid = slice_item(key, item);
        }
        if key == "cid" {
            passport.cid = slice_item(key, item);
        }
    }
}

fn slice_item(key: &str, item: &String) -> String {
    let mut start = item.find(key).ok_or_else(|| 0 as usize).unwrap() + key.len();
    let vec: Vec<char> = item.chars().collect();

    // Move up by one, to deal with ':'
    start += 1;
    let mut end = item.len();

    for i in start..item.len() {
        if vec[i] == ' ' {
            end = i;
            break;
        }
    }

    let string: String = vec[start..end].to_vec().into_iter().collect();
    return string;
}

fn check_passport(passport: &Passport) -> bool {
    if passport.byr.is_empty() {
        return false;
    }

    if passport.iyr.is_empty() {
        return false;
    }

    if passport.eyr.is_empty() {
        return false;
    }

    if passport.hgt.is_empty() {
        return false;
    }

    if passport.hcl.is_empty() {
        return false;
    }

    if passport.ecl.is_empty() {
        return false;
    }

    if passport.pid.is_empty() {
        return false;
    }

    // This does not really matter right now
    // if passport.cid.is_empty() {
    //     return false;
    // }

    return true;
}

fn verify_passport(passport: &Passport) -> bool {
    // byr verification
    let byr = passport.byr.parse::<i32>().unwrap_or(0);
    if passport.byr.len() != 4 || byr < 1920 || byr > 2002 {
        return false;
    }

    // iyr verification
    let iyr = passport.iyr.parse::<i32>().unwrap_or(0);
    if passport.iyr.len() != 4 || iyr < 2010 || iyr > 2020 {
        return false;
    }

    // eyr verification
    let eyr = passport.eyr.parse::<i32>().unwrap_or(0);
    if passport.eyr.len() != 4 || eyr < 2020 || eyr > 2030 {
        return false;
    }

    // hgt/height verification
    match passport.hgt.find("cm") {
        Some(x) => {
            let hgt: String = passport.hgt.chars().collect::<Vec<char>>()[0..x]
                .into_iter()
                .collect();
            let hgt = hgt.parse::<i32>().unwrap_or(0);
            if hgt < 150 || hgt > 193 {
                return false;
            }
        }
        _ => match passport.hgt.find("in") {
            Some(_) => {}
            None => {
                return false;
            }
        },
    }
    match passport.hgt.find("in") {
        Some(x) => {
            let hgt: String = passport.hgt.chars().collect::<Vec<char>>()[0..x]
                .into_iter()
                .collect();
            let hgt = hgt.parse::<i32>().unwrap_or(0);
            if hgt < 59 || hgt > 76 {
                return false;
            }
        }
        _ => match passport.hgt.find("cm") {
            Some(_) => {}
            None => {
                return false;
            }
        },
    }

    // hcl verification
    let hcl_chars = passport.hcl.chars().collect::<Vec<char>>();
    if hcl_chars[0] != '#' {
        return false;
    }
    // exactly 6 characters after the '#'
    if hcl_chars.len() != 7 {
        return false;
    }

    for i in 1..hcl_chars.len() {
        if !hcl_chars[i].is_ascii_hexdigit() {
            return false;
        }
    }

    // ecl verification
    if passport.ecl != "amb"
        && passport.ecl != "blu"
        && passport.ecl != "brn"
        && passport.ecl != "gry"
        && passport.ecl != "grn"
        && passport.ecl != "hzl"
        && passport.ecl != "oth"
    {
        return false;
    }

    // pid verification
    if passport.pid.len() != 9 {
        return false;
    }
    let passport_chars = passport.pid.chars().collect::<Vec<char>>();
    for i in 0..passport.pid.len() {
        if !passport_chars[i].is_ascii_digit() {
            return false;
        }
    }

    // dbg!(&passport.hcl);

    return true;
}

fn main() {
    let filename = "./src/input";
    let content = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let split_content = content.split("\n");

    let mut store: Vec<String> = Vec::new();
    let mut empty_indexes: Vec<usize> = Vec::new();
    let mut i = 0;
    for s in split_content {
        if s.is_empty() {
            empty_indexes.push(i);
        }

        store.push(s.to_string());
        i += 1;
    }

    let mut items: Vec<Vec<String>> = Vec::new();
    let mut previous = 0;
    for i in 0..empty_indexes.len() {
        items.push(store[previous..empty_indexes[i]].to_vec());
        previous = empty_indexes[i] + 1;
    }

    let mut valid = 0;
    for vec_item in items {
        let mut passport = Passport::new();
        for item in vec_item {
            update_passport("byr".into(), &item.to_string(), &mut passport);
            update_passport("iyr".into(), &item.to_string(), &mut passport);
            update_passport("eyr".into(), &item.to_string(), &mut passport);
            update_passport("hgt".into(), &item.to_string(), &mut passport);
            update_passport("hcl".into(), &item.to_string(), &mut passport);
            update_passport("ecl".into(), &item.to_string(), &mut passport);
            update_passport("pid".into(), &item.to_string(), &mut passport);
            update_passport("cid".into(), &item.to_string(), &mut passport);
        }

        if check_passport(&passport) && verify_passport(&passport) {
            valid += 1;
        }
    }

    println!("Total valid passports : {}", valid);
}
