use std::{cell::RefCell, fs::File, rc::Rc};

use anyhow::{anyhow, Result};
use wasmi::*;

pub fn prepare_func_results(ty: &FuncType) -> Box<[Value]> {
    ty.results().iter().copied().map(Value::default).collect()
}

fn load_from_file(filename: &str) -> Vec<u8> {
    use std::io::prelude::*;
    let mut file = File::open(filename).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    buf
}

pub fn main() -> Result<()> {
    // First step is to create the Wasm execution engine with some config.
    // In this example we are using the default configuration.
    let engine = Engine::default();
    let tracer = Tracer::new();
    let tracer = Rc::new(RefCell::new(tracer));

    // Wasmi does not yet support parsing `.wat` so we have to convert
    // out `.wat` into `.wasm` before we compile and validate it.

    let wasm = load_from_file("crates/wasmi/tests/wasms/test_rust.wasm");
    let module = Module::new(&engine, &mut &wasm[..])?;
    // All Wasm objects operate within the context of a `Store`.
    // Each `Store` has a type parameter to store host-specific data,
    // which in this case we are using `42` for.
    type HostState = u32;
    let mut store = Store::new(&engine, 0);
    let host_hello = Func::wrap(&mut store, |caller: Caller<'_, HostState>, param: i32| {
        println!("Got {param} from WebAssembly");
        println!("My host state is: {}", caller.data());
    });

    // In order to create Wasm module instances and link their imports
    //  and exports we require a `Linker`.
    // let mut linker = <Linker<HostState>>::new(&engine);
    let mut linker = <Linker<HostState>>::new(&engine);
    // Instantiation of a Wasm module requires defining its imports and then
    // afterwards we can fetch exports by name, as well as asserting the
    // type signature of the function with `get_typed_func`.
    //
    // Also before using an instance created this way we need to start it.
    linker.define("host", "main", host_hello)?;
    let instance = linker.instantiate(&mut store, &module)?.start(&mut store)?;
    let hello = instance.get_func(&store, "main").unwrap();
    // Prepare output
    let mut ty = prepare_func_results(&hello.ty(&store));
    // And finally we can call the wasm!
    hello.call_with_trace(&mut store, &[], &mut ty, tracer.clone())?;

    println!("{:?}", (*tracer).borrow().etable);

    Ok(())
}
