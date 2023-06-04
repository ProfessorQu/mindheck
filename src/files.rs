use std::{fs::File, io::Read};

pub fn load_file(filename: &String) -> std::io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}