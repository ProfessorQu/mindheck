use std::{fs::File, io::Write};

use crate::files::load_file;

use crate::compiler::functions::*;

use super::Info;

pub fn compile_fn(filename: &String) -> std::io::Result<()> {
    let contents = load_file(filename)?;
    let tokens: Vec<&str> = contents
        .split(|c| c == '\n' || c == '(' || c == ')')
        .collect();

    let mut info = Info::new(tokens);

    while info.tokens_remaining() {
        match info
            .get_token(info.i)
            .expect("Failed to get operator")
            .as_str()
        {
            "print" => print_fn(&mut info),
            "move_to" => move_to_fn(&mut info),
            "add" => add_fn(&mut info),
            "sub" => sub_fn(&mut info),
            "mult" => mult_fn(&mut info),
            x => info.add(x),
        };

        info.inc_i();
    }

    let filename_no_extension = filename.split('.').next().expect("File has no name");
    let mut new_file = File::create(filename_no_extension.to_owned() + ".bf")?;
    new_file.write_all(info.get_result().as_bytes())?;

    Ok(())
}
