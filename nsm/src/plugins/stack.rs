
use crate::Command;
use crate::Processor;
use crate::Plugin;
use crate::Result;

#[derive(Default)]
pub struct Stack;

impl Plugin for Stack {
    fn operations(&self) -> Vec<String> {
        return vec![
            "SET".to_owned(),
        ];
    }
    fn execute(&mut self, proc: &mut Processor, cmd: &Command) -> Result<()> {
        match cmd.name.as_str() {
            "SET" => {
                let value = cmd.params.get(0).cloned()
                    .ok_or_else(|| "SET must have one param".to_owned())?;
                proc.push(value);
            }
            _ => unreachable!(),
        }
        Ok(())
    }
}


