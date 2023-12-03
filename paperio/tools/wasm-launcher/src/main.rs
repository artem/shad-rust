use wasmtime::{Engine, Linker, Module, Store};
use wasmtime_wasi::WasiCtxBuilder;

pub fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    assert_eq!(
        args.len(),
        2,
        "expected exactly 1 argument: a path to a wasm file"
    );
    let wasm_file = &args[1];

    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s| s).unwrap();

    let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().build();
    let mut store = Store::new(&engine, wasi_ctx);

    let module = Module::from_file(&engine, wasm_file).expect("failed to load wasm module");
    linker.module(&mut store, "", &module).unwrap();

    linker
        .get_default(&mut store, "")
        .unwrap()
        .typed::<(), ()>(&store)
        .unwrap()
        .call(&mut store, ())
        .unwrap();
}
