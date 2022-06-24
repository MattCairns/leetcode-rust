use std::collections::{HashMap, VecDeque};

struct FreqStack {
    values: VecDeque<i32>,
    freq_counter: HashMap<i32, u32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FreqStack {
    fn new() -> Self {
        Self {
            values: VecDeque::default(),
            freq_counter: HashMap::default(),
        }
    }

    fn push(&mut self, val: i32) {
        let cnt = self.freq_counter.get(&val).unwrap_or_else(|| &0) + 1 as u32;
        self.values.push_front(val);
        self.freq_counter.insert(val, cnt);
    }

    fn pop(&self) -> i32 {
        let mut max = 0;
        for (k, v) in self.freq_counter.iter().enumerate() {
        }
        5
    }
}

/**
 * Your FreqStack object will be instantiated and called as such:
 * let obj = FreqStack::new();
 * obj.push(val);
 * let ret_2: i32 = obj.pop();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let mut d = FreqStack::new();
        d.push(5);
        d.push(4);
        d.push(5);
        d.push(1);
    }
}
