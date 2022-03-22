
use crate::Value;

/// Single processor for execute tape
#[derive(Default)]
pub struct Processor {
    stack: Vec<Value>,
}

impl Processor {
    /// Read length of stack
    pub fn len(&self) -> usize {
        self.stack.len()
    }
    /// Push value on stack
    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }
    /// Remove and take top value from the stack
    pub fn pop(&mut self) -> Result<Value, String> {
        self.stack.pop()
            .ok_or_else(|| "Stack is empty".to_owned())
    }
    /// Remove and take n top values from the stack
    pub fn take(&mut self, count: usize) -> Result<Vec<Value>, String> {
        let mut res = Vec::with_capacity(count);
        for _ in 0..count {
            let value = self.stack.pop()
                .ok_or_else(|| format!("Stack is empty"))?;
            res.push(value);
        }
        Ok(res)
    }
}

