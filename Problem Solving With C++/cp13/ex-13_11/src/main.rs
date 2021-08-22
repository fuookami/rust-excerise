#![feature(let_chains)]

#[derive(Clone, Copy)]
enum Operator {
    Operator(char),
    Operation(i64),
}

type Expression = Vec<Operator>;

fn calculate(mut expression: Expression) -> i64 {
    let mut stack = Vec::new();
    for op in &expression {
        match op {
            Operator::Operator(operator) => match operator {
                '+' | '-' | '*' | '/' => {
                    stack.push(Operator::Operator(*operator));
                }
                '(' => {}
                ')' => {
                    let _op1 = *expression.last().unwrap();
                    expression.pop();
                    let _op = *expression.last().unwrap();
                    expression.pop();
                    let _op2 = *expression.last().unwrap();
                    expression.pop();
                    if let Operator::Operation(op1) = _op1 {
                        if let Operator::Operator(op) = _op {
                            if let Operator::Operation(op2) = _op2 {
                                match op {
                                    '+' => {
                                        expression.push(Operator::Operation(op1 + op2));
                                    }
                                    '-' => {
                                        expression.push(Operator::Operation(op1 - op2));
                                    }
                                    '*' => {
                                        expression.push(Operator::Operation(op1 * op2));
                                    }
                                    '/' => {
                                        expression.push(Operator::Operation(op1 / op2));
                                    }
                                    _ => {
                                        panic!("");
                                    }
                                }
                            } else {
                                panic!("");
                            }
                        } else {
                            panic!("");
                        }
                    } else {
                        panic!("");
                    }
                }
                _ => {
                    panic!("");
                }
            },
            Operator::Operation(operation) => {
                stack.push(Operator::Operation(*operation));
            }
        }
    }
    let Operator::Operation(ret) = *stack.last().unwrap();
    ret
}

fn main() {
    println!("Hello, world!");
}
