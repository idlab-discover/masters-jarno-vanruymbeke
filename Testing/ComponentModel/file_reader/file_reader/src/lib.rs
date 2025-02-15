#[allow(warnings)]
mod bindings;

use crate::bindings::exports::example::file_reader::reader::Guest;

struct Component;

impl Guest for Component {
    fn read(filename: String) -> String {
        match std::fs::read_to_string(filename.clone()) {
            Ok(content) => content,
            Err(e) => format!("Failed to open file {}: {e}", filename),
        }
    }
}

bindings::export!(Component with_types_in bindings);
