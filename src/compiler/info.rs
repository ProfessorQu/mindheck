pub struct Info {
    tokens: Vec<String>,
    result: String,
    pub pointer: usize,
    pub i: usize,
}

impl Info {
    pub fn new(tokens: Vec<&str>) -> Self {
        Self {
            tokens: tokens.iter().map(|s| s.to_string()).collect(),
            result: String::new(),
            pointer: 0,
            i: 0,
        }
    }

    pub fn move_pointer_to(&mut self, target: usize) {
        let movement = target as i32 - self.pointer as i32;

        if movement > 0 {
            self.add(&">".repeat(movement as usize));
            self.move_pointer(movement as usize);
        } else {
            self.add(&"<".repeat(-movement as usize));
            self.move_pointer(-movement as usize);
        }

        self.add("\n");
    }

    pub fn check_next_is_int(&mut self) -> Option<usize> {
        if let Some(token) = self.get_token(self.i + 1) {
            if let Ok(target) = token.parse::<usize>() {
                self.inc_i();

                return Some(target);
            }
        }

        None
    }

    pub fn check_next_are_ints(&mut self) -> Option<Vec<usize>> {
        let mut ints = vec![];
        if let Some(token) = self.get_token(self.i + 1) {
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
            self.inc_i();

            Some(ints)
        } else {
            None
        }
    }

    pub fn tokens_remaining(&self) -> bool {
        self.i < self.tokens.len()
    }

    pub fn get_token(&self, index: usize) -> Option<String> {
        self.tokens.get(index).cloned()
    }

    pub fn add(&mut self, code: &str) {
        self.result += code;
    }

    pub fn move_pointer(&mut self, movement: usize) {
        self.pointer += movement;
    }

    pub fn inc_i(&mut self) {
        self.i += 1;
    }

    pub fn get_result(self) -> String {
        self.result
    }
}
