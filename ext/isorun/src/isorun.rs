#[magnus::wrap(class = "Isorun::VM")]
pub(crate) struct VM {}

/// SAFETY: This is safe because we only access this data when the GVL is held.
unsafe impl Send for VM {}

impl VM {
    pub fn new() -> VM {
        VM {}
    }

    pub fn run(&self, source: String) -> String {
        let platform = v8::new_default_platform(0, false).make_shared();
        v8::V8::initialize_platform(platform);
        v8::V8::initialize();

        let isolate = &mut v8::Isolate::new(Default::default());

        let scope = &mut v8::HandleScope::new(isolate);
        let context = v8::Context::new(scope);
        let scope = &mut v8::ContextScope::new(scope, context);

        let code = v8::String::new(scope, source.as_str()).unwrap();

        let script = v8::Script::compile(scope, code, None).unwrap();
        let result = script.run(scope).unwrap();
        let result = result.to_string(scope).unwrap();

        result.to_rust_string_lossy(scope)
    }
}
