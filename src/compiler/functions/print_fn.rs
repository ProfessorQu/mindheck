use crate::compiler::{utils::*, Info};

pub fn print_fn(info: &mut Info) {
    if let Some(token) = info.get_token(info.i + 1) {
        if token.starts_with('"') && token.ends_with('"') {
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

                info.add(".\n");
                prev = cur;
            }

            info.add(">");
            info.inc_i();
        } else if let Ok(target) = token.parse::<usize>() {
            move_pointer_to(info, target);
            info.add(".");
            move_pointer_to(info, target);

            info.inc_i();
        } else {
            info.add(".\n");
        }
    } else {
        info.add(".\n");
    }
}
