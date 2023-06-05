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
