use wasi::io::poll;
use wasmtime::component::bindgen;

pub struct DigitalOutPin {}
pub struct DigitalInPin {}
pub struct DigitalInputOutPin {}
pub struct AnalogInPin {}
pub struct AnalogOutPin {}

pub struct Pollable {}

bindgen!({
    world: "gpio",
    path: "../../wit",
    with: {
        "example:gpio/digital/digital-out-pin": DigitalOutPin,
        "example:gpio/digital/digital-in-pin": DigitalInPin,
        "example:gpio/digital/digital-in-out-pin": DigitalInputOutPin,
        "example:gpio/analog/analog-in-pin": AnalogInPin,
        "example:gpio/analog/analog-out-pin": AnalogOutPin,
        "wasi:io/poll/pollable": Pollable
    }
});

impl poll::HostPollable for Pollable {
    #[doc = " Return the readiness of a pollable. This function never blocks."]
#[doc = " "]
#[doc = " Returns `true` when the pollable is ready, and `false` otherwise."]
fn ready(&mut self,self_:wasmtime::component::Resource<Pollable>,) -> bool {
        todo!()
    }

    #[doc = " `block` returns immediately if the pollable is ready, and otherwise"]
#[doc = " blocks until ready."]
#[doc = " "]
#[doc = " This function is equivalent to calling `poll.poll` on a list"]
#[doc = " containing only this pollable."]
fn block(&mut self,self_:wasmtime::component::Resource<Pollable>,) -> () {
        todo!()
    }

    fn drop(&mut self,rep:wasmtime::component::Resource<Pollable>) -> wasmtime::Result<()> {
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
}
