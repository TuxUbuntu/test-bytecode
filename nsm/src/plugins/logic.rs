
//! Plugin for StateMachine with logical and Ord operations
//!
//! Logic can evaluate these commands:
//! * `OR` -- take two top values and set `A OR B`
//! * `AND` -- take two top values and set `A AND B`
//! * `GTE` -- take two top values and set `A >= B`
//! * `LTE` -- take two top values and set `A <= B`
//! * `GT` -- take two top values and set `A > B`
//! * `LT` -- take two top values and set `A < B`
//! * `NEQ` -- take two top values and set `A != B`
//! * `EQ` -- take two top values and set `A == B`
//! * `NOT` -- take top value and set `!A`

use crate::Command;
use crate::Processor;
use crate::Plugin;
use crate::Value;
use crate::Result;

#[derive(Default)]
pub struct Logic;

impl Plugin for Logic {
    fn operations(&self) -> Vec<String> {
        return vec![
            "OR".to_owned(),
            "AND".to_owned(),
            "GTE".to_owned(),
            "LTE".to_owned(),
            "GT".to_owned(),
            "LT".to_owned(),
            "NEQ".to_owned(),
            "EQ".to_owned(),
            "NOT".to_owned(),
        ];
    }
    fn execute(&mut self, proc: &mut Processor, cmd: &Command, position: usize) -> Result<usize> {
        match cmd.name.as_str() {
            "OR" => {
                let values = proc.take(2)?;
                let result = values.into_iter()
                    .map(|a| Ok(a))
                    .map(|a: Result<Value, String>| {
                        if let Value::Boolean(a) = a? {
                            Ok(a)
                        } else {
                            Err("AND must get only boolean".to_owned())
                        }
                    })
                    .reduce(|a, b| Ok(a? || b?))
                    .unwrap()?;
                proc.push(Value::Boolean(result));
            }
            "AND" => {
                let values = proc.take(2)?;
                let result = values.into_iter()
                    .map(|a| Ok(a))
                    .map(|a: Result<Value, String>| {
                        if let Value::Boolean(a) = a? {
                            Ok(a)
                        } else {
                            Err("AND must get only boolean".to_owned())
                        }
                    })
                    .reduce(|a, b| Ok(a? && b?))
                    .unwrap()?;
                proc.push(Value::Boolean(result));
            }
            "NOT" => {
                let value = proc.pop()?;
                let value = if let Value::Boolean(value) = value {
                    value
                } else {
                    return Err("NOT must get only boolean".into());
                };
                proc.push(Value::Boolean(!value));
            }
            "GTE" => {
                let values = proc.take(2)?;
                let result = values.into_iter()
                    .reduce(|a, b| Value::Boolean(a >= b))
                    .unwrap();
                proc.push(result);
            }
            "LTE" => {
                let values = proc.take(2)?;
                let result = values.into_iter()
                    .reduce(|a, b| Value::Boolean(a <= b))
                    .unwrap();
                proc.push(result);
            }
            "GT" => {
                let values = proc.take(2)?;
                let result = values.into_iter()
                    .reduce(|a, b| Value::Boolean(a > b))
                    .unwrap();
                proc.push(result);
            }
            "LT" => {
                let values = proc.take(2)?;
                let result = values.into_iter()
                    .reduce(|a, b| Value::Boolean(a < b))
                    .unwrap();
                proc.push(result);
            }
            "NEQ" => {
                let values = proc.take(2)?;
                let result = values.into_iter()
                    .reduce(|a, b| Value::Boolean(a != b))
                    .unwrap();
                proc.push(result);
            }
            "EQ" => {
                let values = proc.take(2)?;
                let result = values.into_iter()
                    .reduce(|a, b| Value::Boolean(a == b))
                    .unwrap();
                proc.push(result);
            }
            _ => unreachable!(),
        }
        Ok(position + 1)
    }
}


