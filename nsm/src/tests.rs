#![cfg(test)]

use crate::plugins::Memory;
use crate::plugins::Arithmetic;
use crate::plugins::Stack;
use crate::Value;
use crate::Main;
use crate::Command;

#[test]
fn first_test_from_tasklist() {
    let tape = vec![
        Command { name: "SET".to_owned(), params: vec![ Value::Number(2) ] },
        Command { name: "WRITE".to_owned(), params: vec![ Value::String("x".to_owned()) ] },

        Command { name: "SET".to_owned(), params: vec![ Value::Number(3) ] },
        Command { name: "WRITE".to_owned(), params: vec![ Value::String("y".to_owned()) ] },

        Command { name: "READ".to_owned(), params: vec![ Value::String("y".to_owned()) ] },
        Command { name: "SET".to_owned(), params: vec![ Value::Number(1) ] },
        Command { name: "ADD".to_owned(), params: vec![] },
        Command { name: "READ".to_owned(), params: vec![ Value::String("x".to_owned()) ] },
        Command { name: "MULT".to_owned(), params: vec![] },
    ];
    let mut main = Main::default();
    main.register(Memory::default()).unwrap();
    main.register(Stack::default()).unwrap();
    main.register(Arithmetic::default()).unwrap();
    let res = main.read(&tape).unwrap();
    assert_eq!(res, Value::Number(8));
}

