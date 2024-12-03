#[allow(warnings)]
mod bindings;

//use bindings::Guest;

use bindings::wasi::gpio::gpio::{InputPin, OutputPin, StatefulOutputPin};

struct InputPinComponent;

impl InputPin for InputPinComponent {
    
}
//bindings::export!(Component with_types_in bindings);
