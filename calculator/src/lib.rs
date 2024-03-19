#[allow(warnings)]
mod bindings;

mod raw_bindings;

use bindings::exports::component::calculator::calculate::{Guest, Op};

use raw_bindings::component::{add::add, sub::sub};

struct Component;

impl Guest for Component {
    fn eval(op: Op, a: u32, b: u32) -> u32 {
        match op {
            Op::Add => add(a, b),
            Op::Sub => sub(a, b),
        }
    }
}

bindings::export!(Component with_types_in bindings);
