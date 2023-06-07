use std::num::TryFromIntError;

use crate::compiler::Info;

fn print_string(token: String, info: &mut Info) -> Result<(), TryFromIntError> {
    let string = token[1..token.len() - 1].to_owned();

    let mut prev = 0u32;

    for c in string.chars() {
        let cur = c as u32;
        if cur > prev {
            let pluses: usize = (cur - prev).try_into()?;
            info.add(&"+".repeat(pluses));
        } else {
            let minuses: usize = (prev - cur).try_into()?;
            info.add(&"-".repeat(minuses));
        }

        info.add(".");
        prev = cur;
    }

    info.add("-".repeat(prev.try_into()?).as_str());
    print_newline(info)?;

    info.inc_i();

    Ok(())
}

fn print_newline(info: &mut Info) -> Result<(), TryFromIntError> {
    let pointer = info.pointer;

    info.move_pointer_to(300);

    let pluses: usize = ('\n' as u32).try_into()?;
    info.add(&"+".repeat(pluses));

    info.add(".");
    info.add("-".repeat(pluses).as_str());

    info.move_pointer_to(pointer);

    Ok(())
}

pub fn print_fn(info: &mut Info) {
    if let Some(token) = info.get_next_token() {
        if token.starts_with('"') && token.ends_with('"') {
            print_string(token, info).expect("Failed to convert to usize");
        } else if let Some(target) = info.get_tape_target() {
            info.move_pointer_to(target);
            info.add(".");

            print_newline(info).expect("Failed to convert to usize");

            info.inc_i();
        }
    }
}

pub fn input_fn(info: &mut Info) {
    if let Some(token) = info.get_next_token() {
        if token.starts_with('"') && token.ends_with('"') {
            print_string(token, info).expect("Failed to convert to usize");

            info.add(",");
        }
    }
}
