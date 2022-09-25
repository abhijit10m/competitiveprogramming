use std::cmp::Ordering;
#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = Vec::<i32>::new();

    for input in inputs {
        match input {
            CalculatorInput::Value(value) => {stack.push( *value );},
            _ => {
                match stack.len().cmp(&2) {
                    Ordering::Less => { return None}
                    _ => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        match input {
                            CalculatorInput::Add => { stack.push(b + a) },
                            CalculatorInput::Subtract => { stack.push(b - a) },
                            CalculatorInput::Multiply => { stack.push( b * a) },
                            CalculatorInput::Divide => { stack.push(b / a) },
                            _ => {return None}
                        }
                    }
                }
            }
        }
    }
    match stack.len().cmp(&2) {
        Ordering::Less => stack.pop(),
        _ => None
    }
    
}
