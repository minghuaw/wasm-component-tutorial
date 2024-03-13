#[allow(warnings)]
mod bindings;

use bindings::{component::{add::add::add, sub::sub::sub}, exports::component::calculator::calculate::{Guest, Op}};

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
