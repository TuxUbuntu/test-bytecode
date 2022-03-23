#![cfg(test)]

use crate::plugins::Memory;
use crate::plugins::Logic;
use crate::plugins::Arithmetic;
use crate::plugins::Stack;
use crate::plugins::Control;
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
    main.register(Logic::default()).unwrap();
    main.register(Arithmetic::default()).unwrap();
    let res = main.read(&tape).unwrap();
    assert_eq!(res, Value::Number(8));
}

#[test]
fn test_ret() {
    let tape = r#"
        SET 2
        WRITE "x"
        SET 3
        WRITE "y"
        READ "x"
        READ "x"
        READ "y"
        RET
        SET 1
        SUM
        READ "x"
        MULT
    "#.parse().expect("Parse tape error");
    let mut main = StateMachine::default();
    main.register(Memory::default()).unwrap();
    main.register(Stack::default()).unwrap();
    main.register(Logic::default()).unwrap();
    main.register(Control::default()).unwrap();
    main.register(Arithmetic::default()).unwrap();
    let res = main.read(&tape).unwrap();
    assert_eq!(res, Value::Number(3));
}

/// ```
/// x = 10
/// y = 0
/// while x != 0 'a
/// y = y + x
/// x = x - 1
/// end 'a
/// ```
#[test]
fn test_with_loops() {
    let tape = r#"
        // x = 10
        SET 10
        WRITE "x"
        // y = 0
        SET 0
        WRITE "y"
        // loop 'loop_1
        LABEL "loop_1"
        // if x == 0 then 'loop_1_end
        READ "x"
        SET 0
        EQ
        NOT
            JZ "loop_1_end"
            // body of loop_1
            READ "x"
            READ "y"
            SUM
            WRITE "y"
            READ "x"
            SET 1
            SUB
            WRITE "x"
            JUMP "loop_1"
        LABEL "loop_1_end"
        READ "y"
    "#.parse().expect("Parse tape error");
    let mut main = StateMachine::default();
    main.register(Memory::default()).unwrap();
    main.register(Stack::default()).unwrap();
    main.register(Logic::default()).unwrap();
    main.register(Control::default()).unwrap();
    main.register(Arithmetic::default()).unwrap();
    let res = main.read(&tape).unwrap();
    assert_eq!(res, Value::Number(55));
}

