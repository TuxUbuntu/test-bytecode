

use crate::Command;
use crate::Processor;
use crate::Plugin;
use crate::Value;
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
    fn execute(&mut self, proc: &mut Processor, cmd: &Command) -> Result<(), String> {
        match cmd.name.as_str() {
            "READ" => {
                let name = if let Some(Value::String(name)) = cmd.params.get(0).cloned() {
                    name
                } else {
                    return Err("READ must have string param".to_owned());
                };
                let value = self.memory.get(&name).cloned()
                    .ok_or_else(|| format!("READ undefined variable {}", name))?;
                proc.push(value);
            }
            "WRITE" => {
                let name = if let Some(Value::String(name)) = cmd.params.get(0).cloned() {
                    name
                } else {
                    return Err("WRITE must have string param".to_owned());
                };
                let value = proc.pop()?;
                self.memory.insert(name, value);
            }
            _ => unreachable!(),
        }
        Ok(())
    }
}

