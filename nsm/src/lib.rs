
//! Using state machine:
//!
//! ```rust
//! use nsm::*;
//! // initialize instance of StateMachine
//! let mut machine = StateMachine::default();
//! // Add support for `SET` command
//! machine.register(plugins::Stack::default());
//! // Add support for `SUM`,`SUB`,`DIV`,`MULT` commands
//! machine.register(plugins::Arithmetic::default());
//! // Add support for `READ`, `WRITE` commands
//! machine.register(plugins::Memory::default());
//! // x = 2
//! // y = 2
//! // return x + y
//! let tape = r#"
//!     SET 2
//!     WRITE "X"
//!     SET 2
//!     WRITE "Y"
//!     READ "X"
//!     READ "Y"
//!     SUM
//! "#;
//! // parse tape from string
//! let tape = tape.parse().expect("Wrong tape syntax");
//! // evaluate tape on our instance
//! let res = machine.read(&tape).expect("Tape evaluate error");
//! // check result
//! assert_eq!(res, Value::Number(4));
//! ```
//!

mod tests;
mod tape;
mod error;

/// Plugins for state machine
pub mod plugins;
mod processor;

use std::collections::HashMap;
pub use crate::processor::Processor;
pub use crate::tape::Tape;
pub use crate::error::Error;

type Result<T, E = Error> = std::result::Result<T, E>;

/// Trait for make additional functionality for state machine
pub trait Plugin {
    /// Declare list of supported operations
    fn operations(&self) -> Vec<String>;
    /// Execute single command
    /// return new position
    fn execute(&mut self, proc: &mut Processor, cmd: &Command, position: usize) -> Result<usize>;
    /// Preview tape
    fn preview(&mut self, _tape: &Tape) -> Result<()> {
        Ok(())
    }
}

/// Atomic command with params
#[derive(Debug)]
pub struct Command {
    /// Name of command, must be in UPPER_CASE
    pub name: String,
    /// List of params for this command
    pub params: Vec<Value>,
}

/// Naive State Machine
#[derive(Default)]
pub struct StateMachine {
    /// Processor for execute tapes
    proc: Processor,
    /// Index of supported commands
    index: HashMap<String, usize>,
    /// List of plugins for this instance
    plugins: Vec<Box<dyn Plugin>>,
}

impl StateMachine {
    /// Register plugin for current instance
    pub fn register<P>(&mut self, plugin: P) -> Result<()>
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
    /// Read tape and execute it
    pub fn read(&mut self, tape: &Tape) -> Result<Value> {
        for plugin in self.plugins.iter_mut() {
            plugin.preview(tape)?;
        }
        let last = tape.inner.len();
        let mut i = 0;
        while i < last {
            i = self.step(&tape.inner[i], i)?;
        }
        match self.proc.len() {
            0 => Ok(Value::Null),
            1 => Ok(self.proc.pop().unwrap()),
            n @ _ => Err(format!("Stack tail is too long: {}", n).into()),
        }
    }
    /// Do single step
    fn step(&mut self, cmd: &Command, position: usize) -> Result<usize> {
        let id = self.index.get(&cmd.name).cloned()
            .ok_or_else(|| format!("Unknown command: {}", cmd.name))?;
        let plugin = &mut self.plugins[id];
        plugin.execute(&mut self.proc, &cmd, position)
    }
}

/// Value for using in Tape
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Value {
    String(String),
    Number(isize),
    Boolean(bool),
    Null,
}

