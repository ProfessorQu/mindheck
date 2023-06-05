use super::Info;

pub fn move_pointer_to(info: &mut Info, target: usize) {
    let movement = target as i32 - info.pointer as i32;

    if movement > 0 {
        info.add(&">".repeat(movement as usize));
        info.move_pointer(movement as usize);
    } else {
        info.add(&"<".repeat(-movement as usize));
        info.move_pointer(-movement as usize);
    }
}

pub fn check_next_is_int(info: &mut Info) -> Option<usize> {
    if let Some(token) = info.get_token(info.i + 1) {
        if let Ok(target) = token.parse::<usize>() {
            info.inc_i();
            return Some(target);
        }
    }

    None
}

pub fn check_next_are_ints(info: &mut Info) -> Option<Vec<usize>> {
    let mut ints = vec![];
    if let Some(token) = info.get_token(info.i + 1) {
        let token = token.split_whitespace().collect::<Vec<&str>>().join("");

        for num in token.split(',') {
            if let Ok(num) = num.parse::<usize>() {
                ints.push(num);
            } else {
                return None;
            }
        }
    }

    if !ints.is_empty() {
        info.inc_i();

        Some(ints)
    } else {
        None
    }
}
