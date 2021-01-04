use std::fs;
use std::process::exit;
use std::vec::Vec;

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

fn check_passport(passport: Passport) -> bool {
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

        let accept = check_passport(passport);
        if accept {
            valid += 1;
        }
    }

    println!("Total valid passports : {}", valid);
}
