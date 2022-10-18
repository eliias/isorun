#[magnus::wrap(class = "Isorun::VM")]
#[derive(Default)]
pub(crate) struct VM {}

/// SAFETY: This is safe because we only access this data when the GVL is held.
unsafe impl Send for VM {}

impl VM {
    pub fn new() -> VM {
        VM {}
    }

    pub fn run(&self) {
        // let isolate = &mut v8::Isolate::new(Default::default());
        //
        // let scope = &mut v8::HandleScope::new(isolate);
        // let context = v8::Context::new(scope);
        // let scope = &mut v8::ContextScope::new(scope, context);
        //
        // let code = v8::String::new(scope, "'Hello' + ' World!'").unwrap();
        // println!("javascript code: {}", code.to_rust_string_lossy(scope));
        //
        // let script = v8::Script::compile(scope, code, None).unwrap();
        // let result = script.run(scope).unwrap();
        // let result = result.to_string(scope).unwrap();
        // println!("result: {}", result.to_rust_string_lossy(scope));
    }
}
