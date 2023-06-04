use crate::compiler::utils::*;

pub fn print_fn(tokens: &[&str], result: &mut String, i: &mut usize) {
    if let Some(token) = get_token(tokens, *i + 1) {
        if token.starts_with('"') && token.ends_with('"') {
            let string = token[1..token.len() - 1].to_owned();

            let mut prev = 0u32;

            for c in string.chars() {
                let cur = c as u32;
                if cur > prev {
                    let pluses: usize = (cur - prev).try_into().unwrap();
                    *result += &"+".repeat(pluses);
                } else {
                    let minuses: usize = (prev - cur).try_into().unwrap();
                    *result += &"-".repeat(minuses);
                }

                *result += ".\n";
                prev = cur;
            }

            *result += ">";
            *i += 1;
        } else if let Ok(target) = token.parse::<usize>() {
            let pointer = calc_pointer(result);
            move_pointer(result, target);
            *result += ".";
            move_pointer(result, pointer);

            *i += 1;
        }
    }
}
