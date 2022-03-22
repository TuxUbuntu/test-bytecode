#![cfg(test)]

use crate::plugins::Memory;
use crate::plugins::Arithmetic;
use crate::plugins::Stack;
use crate::Value;
use crate::StateMachine;

#[test]
fn first_test_from_tasklist() {
    let tape = r#"
        SET 2
        WRITE "x"
        SET 3
        WRITE "y"
        READ "y"
        SET 1
        SUM
        READ "x"
        MULT
    "#.parse().expect("Parse tape error");
    let mut main = StateMachine::default();
    main.register(Memory::default()).unwrap();
    main.register(Stack::default()).unwrap();
    main.register(Arithmetic::default()).unwrap();
    let res = main.read(&tape).unwrap();
    assert_eq!(res, Value::Number(8));
}

