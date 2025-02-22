// Language

pub type Value = i32;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Op {
    Push(Value),
    Pop,
    Dup,
    Add,
    Mul,
    Print,
}

#[derive(Debug)]
pub enum TokenizationError {
    UnrecognizedSymbol,
}

#[derive(Debug)]
pub enum InterpError {
    StackUnderflow,
    ExecuteValue,
    ExpectedValue,
}

pub struct Interp {
    stack: Vec<Value>,
}

impl Interp {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Result<Value, InterpError> {
        match self.stack.pop() {
            Some(value) => Ok(value),
            None => Err(InterpError::StackUnderflow),
        }
    }

    pub fn peek(&self) -> Result<Value, InterpError> {
        match self.stack.last().clone() {
            Some(value) => Ok(value.clone()),
            None => Err(InterpError::StackUnderflow),
        }
    }

    pub fn trace(&self) {
        println!("TRACE: Stack = {}", self.stack.len());
        self.stack
            .iter()
            .enumerate()
            .rev()
            .for_each(|(index, item)| {
                println!("\t[{index}] => {item:?}");
            });
    }

    pub fn exec_op(&mut self, op: Op) -> Result<(), InterpError> {
        match op {
            Op::Push(value) => {
                self.stack.push(value);
                Ok(())
            },
            Op::Pop => match self.pop() {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            },
            Op::Add => {
                let a = self.pop()?;
                let b = self.pop()?;
                self.push(a + b);
                Ok(())
            }
            Op::Dup => {
                let op = self.peek()?;
                self.push(op);
                Ok(())
            }
            Op::Mul => {
                let a = self.pop()?;
                let b = self.pop()?;
                self.push(a * b);
                Ok(())
            }
            Op::Print => {
                let op = self.pop();
                println!("Example print: {:?}", op);
                Ok(())
            }
        }
    }

    pub fn exec(&mut self, code: &[Op]) -> Result<(), InterpError> {
        let mut counter = 0;
        code.iter().map(|op| *op).try_for_each(|op| {
            println!("EXEC: [{counter}] => {op:?}");
            self.trace();

            self.exec_op(op)?;

            println!("");
            counter += 1;
            Ok(())
        })
    }
}

pub fn tokenize(string: &str) -> Result<Vec<Op>, TokenizationError> {
    string
        .split_whitespace()
        .into_iter()
        .map(|s| match s {
            "+" => Ok(Op::Add),
            "*" => Ok(Op::Mul),
            "POP" => Ok(Op::Pop),
            "DUP" => Ok(Op::Dup),
            "." => Ok(Op::Print),
            _ => match s.parse::<i32>() {
                Ok(num) => Ok(Op::Push(num)),
                Err(_) => Err(TokenizationError::UnrecognizedSymbol),
            },
        })
        .collect::<Result<Vec<Op>, TokenizationError>>()
}

// Run

pub fn run(string: &str) -> Result<(), ()> {
    match tokenize(string) {
        Ok(code) => {
            let mut interp = Interp::new();
            if let Err(err) = interp.exec(&code) {
                eprintln!("ERROR: Interpretter failed, reason: {err:?}");
            }
        }
        Err(err) => eprintln!("ERROR: Tokenization failed, reason: {err:?}"),
    }
    Err(())
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_plus_one() {
        assert_eq!(1 + 1, 2);
    }

    #[test]
    fn tokens_yes() {
        let src = "1 2 + 3 * .";
        let tokens = vec![
            Op::Push(1),
            Op::Push(2),
            Op::Add,
            Op::Push(3),
            Op::Mul,
            Op::Print,
        ];
        let output = tokenize(src).unwrap();
        assert_eq!(output, tokens);
    }
}
