#[allow(warnings)]
mod bindings;

use crate::bindings::exports::guest::implementation::impl_::Guest;
use crate::bindings::host::implementation::impl_::write;

struct Component;

impl Guest for Component {
    fn run() {
        write("Hello from guest");
    }
}

bindings::export!(Component with_types_in bindings);
