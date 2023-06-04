use crate::compiler::utils::*;

pub fn add_fn(tokens: &[&str], result: &mut String, i: &mut usize) {
    if let Some(num) = check_next_is_int(tokens, i) {
        *result += &"+".repeat(num);
    }
}

pub fn sub_fn(tokens: &[&str], result: &mut String, i: &mut usize) {
    if let Some(num) = check_next_is_int(tokens, i) {
        *result += &"-".repeat(num);
    }
}

pub fn move_fn(tokens: &[&str], result: &mut String, i: &mut usize) {
    if let Some(target) = check_next_is_int(tokens, i) {
        move_pointer(result, target);
    }
}
