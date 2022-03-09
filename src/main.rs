mod interpreter;
mod lexer;
mod node;
mod parser;
mod token;
mod utils;

use crate::interpreter::{CellPosition, Interpreter};
use pyroscope::PyroscopeAgent;
use std::time::Instant;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut agent = PyroscopeAgent::builder("http://localhost:4040", "rust-app")
        .sample_rate(100)
        .tags(&[("Hostname", "pyroscope")])
        .build()?;

    agent.start();

    agent.add_tags(&[("Batch", "first")])?;
    let now = Instant::now();
    let mut itp = Interpreter::new();
    let mut i = 0;
    while i < 10000 {
        let input = "1+2+3+4+5+6+7+8+9+10+11+12+13+14";
        let position = CellPosition {
            sheet: "Sheet1".to_string(),
            row: 1,
            col: 1,
        };
        let _ = itp.compute(input, position);
        // println!("{:?}", v);

        // if let Ok(_) = v {
        //     // println!("{:?}", s);
        // } else {
        //     panic!("{:?}", v);
        // }
        i += 1;
    }
    println!("{:?}", now.elapsed());
    agent.stop();
    Ok(())
}
