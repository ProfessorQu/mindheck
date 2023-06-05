use crate::compiler::{utils::*, Info};

pub fn add_fn(info: &mut Info) {
    if let Some(num) = check_next_is_int(info) {
        info.add(&"+".repeat(num));
    }
}

pub fn sub_fn(info: &mut Info) {
    if let Some(num) = check_next_is_int(info) {
        info.add(&"-".repeat(num));
    }
}

pub fn move_to_fn(info: &mut Info) {
    if let Some(target) = check_next_is_int(info) {
        move_pointer_to(info, target);
    }
}

pub fn mult_fn(info: &mut Info) {
    if let Some(nums) = check_next_are_ints(info) {
        if nums.len() != 2 {
            println!("Not the correct number of arguments!");
            return;
        }

        let var1 = nums[0];
        let var2 = nums[1];

        info.add(&"+".repeat(var1));
        info.add("\n[>");
        info.add(&"+".repeat(var2));
        info.add("<-]");
    }
}
