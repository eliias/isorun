use crate::isorun::utils::{convert_ruby_to_v8, convert_v8_to_ruby};
use deno_core::error::AnyError;
use deno_core::serde_v8::from_v8;
use deno_core::{op, serde_v8, Extension, FsModuleLoader, ModuleId};
use deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_runtime::permissions::Permissions;
use deno_runtime::worker::{MainWorker, WorkerOptions};
use deno_runtime::BootstrapOptions;
use deno_web::BlobStore;
use magnus::block::Proc;
use magnus::gvl::GVLContext;
use magnus::{Error, Value};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::rc::Rc;
use std::string::ToString;
use std::sync::Arc;
use tokio::runtime::Runtime;
use v8::{Global, Local};

fn get_error_class_name(e: &AnyError) -> &'static str {
    deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
}

// const BUNDLE_PATH: &str = "bundle_path";
// const ENTRYPOINT: &str = "entrypoint";
// const MESSAGE_RECEIVER: &str = "message_receiver";
const USER_AGENT: &str = "isorun";

pub(crate) struct Worker {
    runtime: Runtime,
    pub(crate) worker: RefCell<MainWorker>,
    module_map: RefCell<HashMap<String, ModuleId>>,
    ruby_context: RefCell<Option<GVLContext>>,
    ruby_receiver: RefCell<Option<Proc>>,
}

impl Worker {
    pub(crate) fn load_module(&self, path: &str) -> Result<ModuleId, AnyError> {
        let mut module_map = self.module_map.borrow_mut();
        if module_map.contains_key(path) {
            return Ok(*module_map.get(path).unwrap());
        }

        let module_id = {
            let mut worker = self.worker.borrow_mut();

            let module_specifier =
                deno_core::resolve_url_or_path(path).unwrap();
            let module_id = self
                .runtime
                .block_on(worker.preload_side_module(&module_specifier))?;
            self.runtime.block_on(worker.evaluate_module(module_id))?;

            module_id
        };

        module_map.insert(path.to_string(), module_id);

        Ok(module_id)
    }

    pub(crate) fn call(
        &self,
        callee: &Global<v8::Value>,
        args: &[Global<v8::Value>],
    ) -> Result<Value, Error> {
        let promise = {
            let mut worker = self.worker.borrow_mut();
            let mut scope = worker.js_runtime.handle_scope();

            let callee = Local::<v8::Value>::new(&mut scope, callee);
            let callee = Local::<v8::Function>::try_from(callee).unwrap();

            let mut local_args: Vec<Local<v8::Value>> = vec![];
            for arg in args {
                let local_arg = Local::<v8::Value>::new(&mut scope, arg);
                local_args.push(local_arg);
            }
            let receiver = v8::undefined(scope.borrow_mut());
            let promise = callee
                .call(&mut scope, receiver.into(), local_args.as_slice())
                .unwrap();
            Global::<v8::Value>::new(&mut scope, promise)
        };

        let value = {
            let mut worker = self.worker.borrow_mut();
            let value = worker.js_runtime.resolve_value(promise);
            self.runtime.block_on(value).unwrap()
        };

        let value = self.to_ruby(&value).unwrap();

        Ok(value)
    }

    pub(crate) fn to_ruby(&self, value: &Global<v8::Value>) -> Option<Value> {
        let mut worker = self.worker.borrow_mut();
        let mut scope = worker.js_runtime.handle_scope();
        let value = Local::new(&mut scope, value);
        let result = convert_v8_to_ruby(value, &mut scope);

        match result {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }

    pub(crate) fn to_v8(&self, value: Value) -> Option<Global<v8::Value>> {
        let mut worker = self.worker.borrow_mut();
        let mut scope = worker.js_runtime.handle_scope();
        let value = convert_ruby_to_v8(value, &mut scope).unwrap();
        let value = Global::<v8::Value>::new(&mut scope, value);

        Some(value)
    }

    fn send(&self, value: Value) -> Result<Value, Error> {
        if let (Some(ctx), Some(rec)) = (
            // we need to deref as mut as both, context and receiver, are behind
            // an Option, and we need to get access to the actual values
            self.ruby_context.borrow_mut().as_mut(),
            self.ruby_receiver.borrow_mut().as_mut(),
        ) {
            ctx.with_gvl(|| {
                let args: (Value,) = (value,);
                rec.call::<(Value,), Value>(args)
            })?
        } else {
            Err(Error::runtime_error(
                "either ruby context or receiver are not initialized",
            ))
        }
    }
}

impl Default for Worker {
    fn default() -> Self {
        let module_loader = Rc::new(FsModuleLoader);
        let create_web_worker_cb = Arc::new(|_| {
            todo!("Web workers are not supported in the example");
        });
        let web_worker_event_cb = Arc::new(|_| {
            todo!("Web workers are not supported in the example");
        });

        let extension_send = Extension::builder()
            .ops(vec![op_send_to_ruby::decl()])
            .build();
        let mut extensions = vec![extension_send];

        let options = WorkerOptions {
            bootstrap: BootstrapOptions {
                args: vec![],
                cpu_count: 1,
                debug_flag: false,
                enable_testing_features: false,
                locale: v8::icu::get_language_tag(),
                location: None,
                no_color: false,
                is_tty: false,
                runtime_version: "x".to_string(),
                ts_version: "x".to_string(),
                unstable: false,
                user_agent: USER_AGENT.to_string(),
                inspect: false,
            },
            extensions: std::mem::take(&mut extensions),
            startup_snapshot: None,
            unsafely_ignore_certificate_errors: None,
            root_cert_store: None,
            seed: None,
            source_map_getter: None,
            format_js_error_fn: None,
            web_worker_preload_module_cb: web_worker_event_cb.clone(),
            web_worker_pre_execute_module_cb: web_worker_event_cb,
            create_web_worker_cb,
            maybe_inspector_server: None,
            should_break_on_first_statement: false,
            module_loader,
            npm_resolver: None,
            get_error_class_fn: Some(&get_error_class_name),
            cache_storage_dir: None,
            origin_storage_dir: None,
            blob_store: BlobStore::default(),
            broadcast_channel: InMemoryBroadcastChannel::default(),
            shared_array_buffer_store: None,
            compiled_wasm_module_store: None,
            stdio: Default::default(),
        };

        let js_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/call.js");
        let main_module =
            deno_core::resolve_path(&js_path.to_string_lossy()).unwrap();
        let permissions = Permissions::allow_all();
        let mut worker = MainWorker::bootstrap_from_options(
            main_module.clone(),
            permissions,
            options,
        );

        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        runtime.block_on(async {
            let module_id =
                worker.preload_main_module(&main_module).await.unwrap();
            worker
                .evaluate_module(module_id)
                .await
                .expect("cannot evaluate core module");
        });

        Worker {
            runtime,
            module_map: RefCell::from(HashMap::default()),
            worker: RefCell::from(worker),
            ruby_context: RefCell::from(None),
            ruby_receiver: RefCell::from(None),
        }
    }
}

thread_local! {
    pub(crate) static WORKER: Worker = Worker::default();
    pub(crate) static MODULE_MAP: HashSet<ModuleId> = HashSet::default();
}

#[allow(clippy::extra_unused_lifetimes)]
#[op(v8)]
fn op_send_to_ruby<'a>(
    // do not remove the v8:: prefix, otherwise the macro complains
    scope: &mut v8::HandleScope,
    data: serde_v8::Value<'a>,
) -> Result<serde_v8::Value<'a>, AnyError> {
    let value = convert_v8_to_ruby(data.v8_value, scope)?;

    WORKER.with(|worker| {
        worker
            .send(value)
            .map(|v| {
                let v = convert_ruby_to_v8(v, scope).unwrap();
                from_v8(scope, v).unwrap()
            })
            .map_err(|error| {
                AnyError::msg(format!("failed to send to ruby: {}", error))
            })
    })
}
