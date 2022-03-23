
//! Memory plugin for StateMachine
//!
//! Memory can evaluate these commands:
//! * `READ` -- get single param with name of variable and push to stack value of it
//! * `WRITE` -- get single param with name of variable and read value from stack. set readed value
//! in variable

use crate::Command;
use crate::Processor;
use crate::Plugin;
use crate::Value;
use crate::Result;
use std::collections::HashMap;

#[derive(Default)]
pub struct Memory {
    memory: HashMap<String, Value>,
}

impl Plugin for Memory {
    fn operations(&self) -> Vec<String> {
        return vec![
            "READ".to_owned(),
            "WRITE".to_owned(),
        ];
    }
    fn execute(&mut self, proc: &mut Processor, cmd: &Command, position: usize) -> Result<usize> {
        match cmd.name.as_str() {
            "READ" => {
                let name = if let Some(Value::String(name)) = cmd.params.get(0).cloned() {
                    name
                } else {
                    return Err("READ must have string param".into());
                };
                let value = self.memory.get(&name).cloned()
                    .ok_or_else(|| format!("READ undefined variable {}", name))?;
                proc.push(value);
            }
            "WRITE" => {
                let name = if let Some(Value::String(name)) = cmd.params.get(0).cloned() {
                    name
                } else {
                    return Err("WRITE must have string param".into());
                };
                let value = proc.pop()?;
                self.memory.insert(name, value);
            }
            _ => unreachable!(),
        }
        Ok(position + 1)
    }
}

