use std::{fs::File, io::Write};

use crate::compiler::utils::*;
use crate::files::load_file;

use crate::compiler::functions::*;

pub fn compile_fn(filename: &String) -> std::io::Result<()> {
    let contents = load_file(filename)?;
    let tokens: Vec<&str> = contents
        .split(|c| c == '\n' || c == '(' || c == ')')
        .collect();

    let mut result = String::new();

    let mut i = 0;
    while i < tokens.len() {
        match get_token(&tokens, i)
            .expect("Failed to get operator")
            .as_str()
        {
            "print" => print_fn(&tokens, &mut result, &mut i),
            "move" => move_fn(&tokens, &mut result, &mut i),
            "add" => add_fn(&tokens, &mut result, &mut i),
            "sub" => sub_fn(&tokens, &mut result, &mut i),
            x => result += x,
        };

        i += 1;
    }

    let filename_no_extension = filename.split('.').next().expect("File has no name");
    let mut new_file = File::create(filename_no_extension.to_owned() + ".bf")?;
    new_file.write_all(result.as_bytes())?;

    Ok(())
}
