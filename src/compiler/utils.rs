pub fn get_token(operators: &[&str], i: usize) -> Option<String> {
    operators.get(i).map(|operator| operator.to_string())
}

pub fn move_pointer_to(result: &mut String, pointer: &mut usize, target: usize) {
    let movement = target as i32 - *pointer as i32;

    if movement > 0 {
        *result += &">".repeat(movement as usize);
        *pointer += movement as usize;
    } else {
        *result += &"<".repeat(-movement as usize);
        *pointer += -movement as usize;
    }
}

pub fn check_next_is_int(tokens: &[&str], i: &mut usize) -> Option<usize> {
    if let Some(token) = get_token(tokens, *i + 1) {
        if let Ok(target) = token.parse::<usize>() {
            *i += 1;

            return Some(target);
        }
    }

    None
}

pub fn check_next_are_ints(tokens: &[&str], i: &mut usize) -> Option<Vec<usize>> {
    let mut ints = vec![];
    if let Some(token) = get_token(tokens, *i + 1) {
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
        *i += 1;

        Some(ints)
    } else {
        None
    }
}
