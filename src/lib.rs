#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Op {
    Push(i32),
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
}

pub struct Interp {
    stack: Vec<Op>,
}

impl Interp {
    pub fn push(&mut self, op: Op) {
        todo!()
    }

    pub fn pop(&mut self) -> Result<(), InterpError> {
        todo!()
    }

    pub fn exec_op(&mut self, op: Op) -> Result<(), InterpError> {
        todo!()
    }

    pub fn exec(&mut self) -> Result<(), InterpError> {
        todo!()
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
