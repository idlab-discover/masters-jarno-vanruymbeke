use anyhow::Result;
use wasmtime::{component::bindgen, *};
use wasmtime_wasi::WasiCtxBuilder;

// Generate bindings for the host component (from deps/host)
bindgen!({
    path: "../wit/deps/host",  // Path to the host WIT file
    world: "host",  // Matches `world host` in deps/host/world.wit
    trappable_imports: true
});

// Generate bindings for the guest component (from guest.wit)
/*bindgen!({
    path: "../wit",  // Path to guest.wit (now in wit/)
    world: "guest",  // Matches `world guest` in guest.wit
    with: {
        "host:implementation/host-impl": 
    }
    //with: {
    //    "host:implementation/host-impl":,  // Maps guest import to Rust's host binding
    //}
});*/


fn main() -> Result<()> {
    let engine = Engine::new(Config::new().wasm_component_model(true));


    //let linker = Linker::new(&engine);
    //let wasi = WasiCtxBuilder::new().build();
    //let component = component::Component::from_file(&engine, "./guest.wasm");
    exports::host::
    Ok(())
}
