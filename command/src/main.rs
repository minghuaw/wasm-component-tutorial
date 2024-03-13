use clap::Parser;
use std::fmt;

#[allow(warnings)]
mod bindings;

use bindings::component::calculator::calculate::Op;

use crate::bindings::component::calculator::calculate;

fn parse_operator(op: &str) -> anyhow::Result<Op> {
    match op {
        "add" => Ok(Op::Add),
        "sub" => Ok(Op::Sub),
        _ => anyhow::bail!("Unknown operation: {}", op),
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Op::Add => write!(f, "+"),
            Op::Sub => write!(f, "-"),
        }
    }
}

/// A CLI for executing mathematical expressions
/// using WebAssembly
#[derive(Parser)]
#[clap(name = "calculator", version = env!("CARGO_PKG_VERSION"))]
struct Command {
    /// The first operand
    x: u32,
    /// The second operand
    y: u32,
    /// Expression operator
    #[clap(value_parser = parse_operator)]
    op: Op,
}

impl Command {
    fn run(self) {
        let res = calculate::eval(self.op, self.x, self.y);
        println!("{} {} {} = {res}", self.x, self.op, self.y);
    }
}

fn main() {
    Command::parse().run()
}