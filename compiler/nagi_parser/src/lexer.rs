pub(crate) mod code;
pub(crate) mod spec;

#[derive(Debug)]
pub(crate) struct Lexer<T> {
    token_list: Vec<T>,
    token_position: usize,
}

impl<T> Lexer<T> {
    pub fn new(token_list: Vec<T>) -> Self {
        Self {
            token_list,
            token_position: 0,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.token_list.get(self.token_position)
    }

    pub fn next(&mut self) -> Option<&T> {
        let position = self.token_position;
        self.token_position += 1;
        self.token_list.get(position)
    }

    pub fn next_if(&mut self, f: fn(&T) -> bool) -> Option<&T> {
        let t = self.peek()?;
        if !f(t) {
            return None;
        }
        self.next()
    }

    pub fn get_token_position(&self) -> usize {
        self.token_position
    }

    pub fn set_token_position(&mut self, position: usize) {
        self.token_position = position;
    }
}
