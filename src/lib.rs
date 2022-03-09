mod interpreter;
mod lexer;
mod node;
mod parser;
mod token;
mod utils;

pub use crate::interpreter::{CellPosition, Interpreter};

#[cfg(test)]
mod tests {
    use crate::interpreter::{CellPosition, Interpreter};
    use std::time::Instant;

    #[test]
    fn compute() {
        let now = Instant::now();
        let mut itp = Interpreter::new();
        let mut i = 0;
        while i < 10 {
            let input = "1+2+3+4+5+6+7+8+9+10";
            let position = CellPosition {
                sheet: "Sheet1".to_string(),
                row: 1,
                col: 1,
            };
            let _ = itp.compute(input, position);
            i += 1;
        }
        println!("{:?}", now.elapsed());
        // println!("{:?}", result);
    }
}
