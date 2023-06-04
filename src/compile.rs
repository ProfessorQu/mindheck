use std::{fs::File, io::Write};

use crate::files::load_file;

fn get_token(operators: &[&str], i: usize) -> Option<String> {
    operators.get(i).map(|operator| operator.to_string())
}

pub fn compile(filename: &String) -> std::io::Result<()> {
    let mut contents = load_file(filename)?;
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
            "print" => {
                if let Some(token) = get_token(&tokens, i + 1) {
                    if token.starts_with('"') && token.ends_with('"') {
                        let string = token[1..token.len() - 1].to_owned() + "\n";

                        let mut prev = 0u32;

                        for c in string.chars() {
                            let cur = c as u32;
                            if cur > prev {
                                let pluses: usize = (cur - prev).try_into().unwrap();
                                result += &"+".repeat(pluses);
                            }
                            else {
                                let minuses: usize = (prev - cur).try_into().unwrap();
                                result += &"-".repeat(minuses);
                            }

                            result += ".\n";
                            prev = cur;
                        }

                        result += ">";
                    }
                }
            }
            "else" => print!("finally"),
            _ => ()
        };

        i += 1;
    }

    let filename_no_extension = filename.split('.').next().expect("File has no name");
    let mut new_file = File::create(filename_no_extension.to_owned() + ".bf")?;
    new_file.write_all(result.as_bytes())?;

    Ok(())
}
