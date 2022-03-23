
//! Flow control plugin for StateMachine
//!
//! Control can evaluate these commands:
//! * `RET` -- return top value from stack
//! * `LABEL` -- set label for jump
//! * `JUMP` -- move cursor to label
//! * `JZ` -- do `JUMP` if top value is `false`

use crate::Command;
use crate::Processor;
use crate::Plugin;
use crate::Value;
use crate::Tape;
use crate::Result;
use std::collections::HashMap;

#[derive(Default)]
pub struct Control {
    len: usize,
    labels: HashMap<String, usize>,
}

impl Plugin for Control {
    fn operations(&self) -> Vec<String> {
        return vec![
            "RET".to_owned(),
            "LABEL".to_owned(),
            "JUMP".to_owned(),
            "JZ".to_owned(),
        ];
    }
    fn preview(&mut self, tape: &Tape) -> Result<()> {
        let mut position = 0;
        for cmd in tape.inner.iter() {
            position += 1;
            if cmd.name == "LABEL" {
                let label = if let Some(Value::String(label)) = cmd.params.get(0).cloned() {
                    label
                } else {
                    return Err("LABEL must have string param".into());
                };
                self.labels.insert(label, position);
            }
        }
        self.len = position;
        Ok(())
    }
    fn execute(&mut self, proc: &mut Processor, cmd: &Command, position: usize) -> Result<usize> {
        match cmd.name.as_str() {
            "LABEL" => {
                Ok(position + 1)
            }
            "JUMP" => {
                let label = if let Some(Value::String(label)) = cmd.params.get(0).cloned() {
                    label
                } else {
                    return Err("JUMP must have string param".into());
                };
                let pos = self.labels.get(&label)
                    .ok_or_else(|| format!("JUMP to undefined label {}", label))?;
                Ok(*pos)
            }
            "JZ" => {
                let label = if let Some(Value::String(label)) = cmd.params.get(0).cloned() {
                    label
                } else {
                    return Err("JZ must have string param".into());
                };
                let pos = self.labels.get(&label)
                    .ok_or_else(|| format!("JUMP to undefined label {}", label))?;
                let value = proc.pop()?;
                if value == Value::Boolean(false) {
                    Ok(*pos)
                } else {
                    Ok(position + 1)
                }
            }
            "RET" => {
                let value = proc.pop()?;
                proc.take(proc.len())?;
                proc.push(value);
                Ok(self.len + 1)
            }
            _ => unreachable!(),
        }
    }
}

