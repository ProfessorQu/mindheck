use crate::files::load_file;

const TAPE_SIZE: usize = 30000;

fn get_op(contents: &str, i: usize) -> Option<char> {
    contents.chars().nth(i)
}

pub fn run(filename: &String) -> std::io::Result<()> {
    let mut contents = load_file(filename)?;

    contents = contents.split_whitespace().collect();

    let mut tape = [0u32; TAPE_SIZE];

    let mut pointer: usize = 0;

    let stdin = std::io::stdin();
    let mut input = String::new();

    let mut i = 0;
    while i < contents.len() {
        let op = get_op(&contents, i).expect("Failed to get operator");
        match op {
            '>' => pointer = pointer.wrapping_add(1),
            '<' => pointer = pointer.wrapping_sub(1),
            '+' => tape[pointer] = tape[pointer].wrapping_add(1),
            '-' => tape[pointer] = tape[pointer].wrapping_sub(1),
            '.' => {
                if let Some(c) = char::from_u32(tape[pointer]) {
                    print!("{}", c);
                }
            }
            ',' => {
                while input.is_empty() {
                    stdin.read_line(&mut input)?;
                }
                tape[pointer] = input.chars().next().unwrap() as u32 - '0' as u32;
                input.clear();
            }
            '[' => {
                if tape[pointer] == 0 {
                    let mut loop_count = 1;
                    while loop_count > 0 {
                        i += 1;

                        if let Some(next_op) = get_op(&contents, i) {
                            if next_op == '[' {
                                loop_count += 1;
                            } else if next_op == ']' {
                                loop_count -= 1;
                            }
                        }
                    }
                }
            }
            ']' => {
                if tape[pointer] != 0 {
                    let mut loop_count = 1;
                    while loop_count > 0 {
                        i -= 1;

                        if let Some(next_op) = get_op(&contents, i) {
                            if next_op == '[' {
                                loop_count -= 1;
                            } else if next_op == ']' {
                                loop_count += 1;
                            }
                        }
                    }
                }
            }
            _ => (),
        }

        i += 1;
    }

    Ok(())
}
