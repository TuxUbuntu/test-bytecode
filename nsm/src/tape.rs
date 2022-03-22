
use crate::Command;
use crate::Value;
use std::str::FromStr;

/// Tape with commands
pub struct Tape {
    pub(crate) inner: Vec<Command>,
}

impl FromStr for Command {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (head, mut tail) = s.trim().split_once(" ")
            .map(|(a, b)| (a, Some(b))).unwrap_or_else(|| (s, None));
        let name = head.to_owned();
        let mut params = Vec::new();
        while let Some(t) = tail {
            let (head, t) = t.trim().split_once(" ")
                .map(|(a, b)| (a, Some(b))).unwrap_or_else(|| (t, None));
            let value = head.parse()?;
            params.push(value);
            tail = t;
        }
        Ok(Command {
            name,
            params
        })
    }
}

impl FromStr for Tape {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cmds = s.lines()
            .map(str::trim)
            .filter(|s| !s.starts_with("//") && !s.is_empty())
            .map(|s| s.parse())
            .collect::<Result<Vec<Command>, Self::Err>>()?;
        Ok(Tape { inner: cmds })
    }
}

impl FromStr for Value {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let len = s.len();
        let mut chars = s.chars();
        match chars.next() {
            Some('0'..='9') => Ok(Value::Number(s.parse().unwrap())),
            Some('n' | 'N') => Ok(Value::Null),
            Some('t') => Ok(Value::Boolean(true)),
            Some('f') => Ok(Value::Boolean(false)),
            Some('"') if chars.last().unwrap() == '"' =>
                Ok(Value::String(s[1..len-1].to_string())),
            _ => unimplemented!(),
        }
    }
}


