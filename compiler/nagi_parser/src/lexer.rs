use std::{collections::HashMap, hash::Hash};

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

    pub fn next_if<F>(&mut self, mut f: F) -> Option<&T>
    where
        F: FnMut(&T) -> bool,
    {
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

pub(crate) type PatternType<T, U> = Vec<(Vec<T>, U)>;
pub(crate) type PatternHashMap<T, U> = HashMap<T, PatternType<T, U>>;
pub(crate) fn make_pattern_map<T, U>(list: PatternType<T, U>) -> PatternHashMap<T, U>
where
    T: Clone + Hash + Eq + PartialEq,
{
    let mut hash_map: PatternHashMap<T, U> = HashMap::new();
    for (pattern, value) in list {
        let Some(key) = pattern.first().cloned() else {
            unreachable!();
        };
        hash_map.entry(key).or_default().push((pattern, value));
    }

    // 最長マッチするように降順にソート
    for (_, patterns) in hash_map.iter_mut() {
        patterns.sort_by_key(|(p, _)| std::cmp::Reverse(p.len()));
    }

    hash_map
}
