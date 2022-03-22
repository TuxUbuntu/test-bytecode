
mod tests;
pub mod plugins;

use std::collections::HashMap;

pub trait Plugin {
    fn operations(&self) -> Vec<String>;
    fn execute(&mut self, proc: &mut Processor, cmd: &Command) -> Result<(), String>;
}

pub struct Command {
    pub name: String,
    pub params: Vec<Value>,
}

type Tape = Vec<Command>;

#[derive(Default)]
pub struct Processor {
    stack: Vec<Value>,
}

#[derive(Default)]
pub struct Main {
    proc: Processor,
    index: HashMap<String, usize>,
    plugins: Vec<Box<dyn Plugin>>,
}

impl Main {
    pub fn register<P>(&mut self, plugin: P) -> Result<(), String>
    where
        P: Plugin + 'static,
    {
        let id = self.plugins.len();
        for op in plugin.operations() {
            self.index.insert(op, id);
        }
        self.plugins.push(Box::new(plugin));
        Ok(())
    }
    pub fn read(&mut self, tape: &Tape) -> Result<Value, String> {
        for cmd in tape.iter() {
            self.step(cmd)?;
        }
        match self.proc.len() {
            0 => Ok(Value::Null),
            1 => Ok(self.proc.pop().unwrap()),
            n @ _ => Err(format!("Stack tail is too long: {}", n)),
        }
    }
    fn step(&mut self, cmd: &Command) -> Result<(), String> {
        let id = self.index.get(&cmd.name).cloned()
            .ok_or_else(|| format!("Unknown command: {}", cmd.name))?;
        let plugin = &mut self.plugins[id];
        plugin.execute(&mut self.proc, &cmd)?;
        Ok(())
    }
}

impl Processor {
    fn len(&self) -> usize {
        self.stack.len()
    }
    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }
    fn pop(&mut self) -> Result<Value, String> {
        self.stack.pop()
            .ok_or_else(|| "Stack is empty".to_owned())
    }
    fn take(&mut self, count: usize) -> Result<Vec<Value>, String> {
        let mut res = Vec::with_capacity(count);
        for _ in 0..count {
            let value = self.stack.pop()
                .ok_or_else(|| format!("Stack is empty"))?;
            res.push(value);
        }
        Ok(res)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(String),
    Number(isize),
    Boolean(bool),
    Null,
}

