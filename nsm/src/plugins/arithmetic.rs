
use crate::Command;
use crate::Processor;
use crate::Plugin;
use crate::Value;

#[derive(Default)]
pub struct Arithmetic;

impl Plugin for Arithmetic {
    fn operations(&self) -> Vec<String> {
        return vec![
            "ADD".to_owned(),
            "MULT".to_owned(),
            "SUB".to_owned(),
            "DIV".to_owned(),
        ];
    }
    fn execute(&mut self, proc: &mut Processor, cmd: &Command) -> Result<(), String> {
        match cmd.name.as_str() {
            "ADD" => {
                let values = proc.take(2)?;
                let result = values.into_iter()
                    .map(|a| Ok(a))
                    .map(|a: Result<Value, String>| {
                        if let Value::Number(a) = a? {
                            Ok(a)
                        } else {
                            Err("ADD must get only numbers".to_owned())
                        }
                    })
                    .reduce(|a, b| Ok(a? + b?))
                    .unwrap()?;
                proc.push(Value::Number(result));
            }
            "SUB" => {
                let values = proc.take(2)?;
                let result = values.into_iter()
                    .map(|a| Ok(a))
                    .map(|a: Result<Value, String>| {
                        if let Value::Number(a) = a? {
                            Ok(a)
                        } else {
                            Err("SUB must get only numbers".to_owned())
                        }
                    })
                    .reduce(|a, b| Ok(a? - b?))
                    .unwrap()?;
                proc.push(Value::Number(result));
            }
            "MULT" => {
                let values = proc.take(2)?;
                let result = values.into_iter()
                    .map(|a| Ok(a))
                    .map(|a: Result<Value, String>| {
                        if let Value::Number(a) = a? {
                            Ok(a)
                        } else {
                            Err("MULT must get only numbers".to_owned())
                        }
                    })
                    .reduce(|a, b| Ok(a? * b?))
                    .unwrap()?;
                proc.push(Value::Number(result));
            }
            "DIV" => {
                let values = proc.take(2)?;
                let result = values.into_iter()
                    .map(|a| Ok(a))
                    .map(|a: Result<Value, String>| {
                        if let Value::Number(a) = a? {
                            Ok(a)
                        } else {
                            Err("DIV must get only numbers".to_owned())
                        }
                    })
                    .reduce(|a, b| Ok(a? / b?))
                    .unwrap()?;
                proc.push(Value::Number(result));
            }
            _ => unreachable!(),
        }
        Ok(())
    }
}

