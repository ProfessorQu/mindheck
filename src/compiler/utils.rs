pub fn get_token(operators: &[&str], i: usize) -> Option<String> {
    operators.get(i).map(|operator| operator.to_string())
}

pub fn calc_pointer(result: &str) -> usize {
    result.chars().filter(|c| c == &'>').count() - result.chars().filter(|c| c == &'<').count()
}

pub fn move_pointer(result: &mut String, target: usize) {
    let pointer = calc_pointer(result);
    let movement = target as i32 - pointer as i32;

    if movement > 0 {
        *result += &">".repeat(movement as usize);
    } else {
        *result += &"<".repeat(-movement as usize);
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
