use bindings::exports::component::add::add::Guest;

#[allow(warnings)]
mod bindings;

struct Component;

impl Guest for Component {
    fn add(a: u32, b: u32) -> u32 {
        a + b
    }
}

bindings::export!(Component with_types_in bindings);
