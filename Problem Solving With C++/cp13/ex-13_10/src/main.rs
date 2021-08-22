enum Operator {
    Operator(char),
    Operation(i64),
}

type Expression = Vec<Operator>;

fn calculate(expression: &Expression) -> i64 {
    let mut stack = Vec::new();
    for op in expression {
        match op {
            Operator::Operator(operator) => {
                let op1: i64 = *stack.last().unwrap();
                stack.pop();
                let op2: i64 = *stack.last().unwrap();
                stack.pop();
                match operator {
                    '+' => {
                        stack.push(op1 + op2);
                    }
                    '-' => {
                        stack.push(op1 - op2);
                    }
                    '*' => {
                        stack.push(op1 * op2);
                    }
                    '/' => {
                        stack.push(op1 / op2);
                    }
                    _ => {
                        panic!("");
                    }
                }
            }
            Operator::Operation(operation) => {
                stack.push(*operation);
            }
        }
    }
    *stack.last().unwrap()
}

fn main() {
    println!("Hello, world!");
}
