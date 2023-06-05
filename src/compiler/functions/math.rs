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

pub fn move_to_fn(tokens: &[&str], result: &mut String, pointer: &mut usize, i: &mut usize) {
    if let Some(target) = check_next_is_int(tokens, i) {
        move_pointer_to(result, pointer, target);
    }
}

pub fn mult_fn(tokens: &[&str], result: &mut String, i: &mut usize) {
    if let Some(nums) = check_next_are_ints(tokens, i) {
        if nums.len() != 2 {
            println!("Not the correct number of arguments!");
            return;
        }

        let var1 = nums[0];
        let var2 = nums[1];

        *result += &"+".repeat(var1);
        *result += "\n[>";
        *result += &"+".repeat(var2);
        *result += "<-]";
    }
}
