mod bindings;

use bindings::example::file_reader::reader::read;

fn main() {
    let args : Vec<String> = std::env::args().collect();
    println!("{}", read(&args[1]));
}
