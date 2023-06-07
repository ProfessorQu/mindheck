use crate::compiler::Info;

fn print_string(token: String, info: &mut Info) {
    let string = token[1..token.len() - 1].to_owned();

    let mut prev = 0u32;

    for c in string.chars() {
        let cur = c as u32;
        if cur > prev {
            let pluses: usize = (cur - prev).try_into().unwrap();
            info.add(&"+".repeat(pluses));
        } else {
            let minuses: usize = (prev - cur).try_into().unwrap();
            info.add(&"-".repeat(minuses));
        }

        info.add(".");
        prev = cur;
    }

    print_newline(info);

    info.add("[-]");
    info.inc_i();
}

fn print_newline(info: &mut Info) {
    info.add("[-]");
    let pluses: usize = ('\n' as u32).try_into().unwrap();
    info.add(&"+".repeat(pluses));

    info.add(".");
}

pub fn print_fn(info: &mut Info) {
    if let Some(token) = info.get_next_token() {
        if token.starts_with('"') && token.ends_with('"') {
            print_string(token, info);
        } else if let Some(target) = info.get_tape_target() {
            info.move_pointer_to(target);
            info.add(".");

            print_newline(info);

            info.inc_i();
        } else {
            info.add(".");
        }
    } else {
        info.add(".");
    }
}

pub fn input_fn(info: &mut Info) {
    if let Some(token) = info.get_next_token() {
        if token.starts_with('"') && token.ends_with('"') {
            print_string(token, info);

            info.add(",");
        }
    }
}
