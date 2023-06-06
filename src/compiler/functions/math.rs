use crate::compiler::Info;

pub fn add_fn(info: &mut Info) {
    if let Some(num) = info.check_next_is_int() {
        info.add(&"+".repeat(num));
    } else if let Some(target) = info.get_tape_target() {
        let cur = info.pointer;
        info.move_pointer_to(target);

        info.add("[-");
        info.move_pointer_to(cur);

        info.add("+");
        info.move_pointer_to(target);

        info.add("]");

        info.inc_i();
    }
}

pub fn sub_fn(info: &mut Info) {
    if let Some(num) = info.check_next_is_int() {
        info.add("-".repeat(num).as_str());
    }
}

pub fn move_to_fn(info: &mut Info) {
    if let Some(target) = info.check_next_is_int() {
        info.move_pointer_to(target);
    }
}

pub fn mult_fn(info: &mut Info) {
    if let Some(nums) = info.check_next_are_ints() {
        if nums.len() != 2 {
            println!("Not the correct number of arguments!");
            return;
        }

        let var1 = nums[0];
        let var2 = nums[1];

        info.add(&"+".repeat(var1));
        info.add("[>");
        info.add(&"+".repeat(var2));
        info.add("<-]");
    }
}
