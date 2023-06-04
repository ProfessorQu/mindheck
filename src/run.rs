use std::{fs::File, io::Read};

pub fn run(file: &String) {
    let mut file = match File::open(file) {
        Ok(file) => file,
        Err(_) => { println!("Couldn't open file {:?}", file); return }
    };

    let mut contents = String::new();

    if file.read_to_string(&mut contents).is_err() {
        println!("Couldn't read contents of file {:?}", file);
        return;
    }

    contents = contents.split_whitespace().collect();

    for op in contents.chars() {
        println!("{}", op);
    }
}