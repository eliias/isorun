use crate::ext;
use crate::isorun::utils::{convert_ruby_to_v8, convert_v8_to_ruby};
use deno_core::anyhow::Context;
use deno_core::error::AnyError;
use deno_core::serde_v8::from_v8;
use deno_core::{
    serde_v8, CreateRealmOptions, FsModuleLoader, JsRealm, ModuleId,
    ModuleLoader,
};
use deno_fs::RealFs;
use deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_runtime::permissions::PermissionsContainer;
use deno_runtime::worker::{MainWorker, WorkerOptions};
use deno_runtime::BootstrapOptions;
use deno_web::BlobStore;
use magnus::block::Proc;
use magnus::gvl::{without_gvl, GVLContext};
use magnus::{Error, Value};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::env;
use std::path::Path;
use std::rc::Rc;
use std::string::ToString;
use std::sync::Arc;
use tokio::runtime::Runtime;
use v8::{Global, HandleScope, Local};

fn get_error_class_name(e: &AnyError) -> &'static str {
    deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
}

const USER_AGENT: &str = "isorun";

pub(crate) struct Worker {
    pub(crate) runtime: Runtime,
    pub(crate) worker: RefCell<MainWorker>,
    module_map: RefCell<HashMap<String, ModuleId>>,
    pub(crate) ruby_context: RefCell<Option<GVLContext>>,
    pub(crate) ruby_receiver: RefCell<Option<Proc>>,
}

impl Worker {
    pub(crate) fn create_realm(&self) -> Result<JsRealm, AnyError> {
        let mut worker = self.worker.borrow_mut();
        let loader: Option<Rc<dyn ModuleLoader>> =
            Some(Rc::new(FsModuleLoader));
        let options = CreateRealmOptions {
            module_loader: loader,
        };
        worker.js_runtime.create_realm(options)
    }

    pub(crate) fn load_module(&self, path: &str) -> Result<ModuleId, AnyError> {
        let mut module_map = self.module_map.borrow_mut();
        if module_map.contains_key(path) {
            return Ok(*module_map.get(path).unwrap());
        }

        let module_id = {
            let mut worker = self.worker.borrow_mut();

            let module_specifier = deno_core::resolve_url_or_path(
                path,
                &env::current_dir().context("Unable to get CWD")?,
            )
            .unwrap();
            let module_id = self
                .runtime
                .block_on(worker.preload_side_module(&module_specifier))?;
            self.runtime.block_on(worker.evaluate_module(module_id))?;

            module_id
        };

        module_map.insert(path.to_string(), module_id);

        Ok(module_id)
    }

    pub(crate) async fn call(
        &self,
        realm: &JsRealm,
        callee: &Global<v8::Value>,
        args: &[Global<v8::Value>],
    ) -> Result<Value, Error> {
        let promise = {
            let mut worker = self.worker.borrow_mut();
            let mut scope = realm.handle_scope(worker.js_runtime.v8_isolate());

            let callee = Local::<v8::Value>::new(&mut scope, callee);
            let callee = Local::<v8::Function>::try_from(callee).unwrap();

            let mut local_args: Vec<Local<v8::Value>> = vec![];
            for arg in args {
                let local_arg = Local::<v8::Value>::new(&mut scope, arg);
                local_args.push(local_arg);
            }
            let receiver = v8::undefined(scope.borrow_mut());
            let (promise, _cancel) = without_gvl(
                |ctx| {
                    self.ruby_context.replace(Some(ctx));
                    let result = callee
                        .call(
                            &mut scope,
                            receiver.into(),
                            local_args.as_slice(),
                        )
                        .unwrap();
                    self.ruby_context.replace(None);
                    result
                },
                None::<fn()>,
            );

            let promise = promise.unwrap();
            Global::<v8::Value>::new(&mut scope, promise)
        };

        let value = {
            let mut worker = self.worker.borrow_mut();
            worker.js_runtime.resolve_value(promise).await.unwrap()
        };

        let value = self.to_ruby(realm, &value).unwrap();

        Ok(value)
    }

    pub(crate) fn to_ruby(
        &self,
        realm: &JsRealm,
        value: &Global<v8::Value>,
    ) -> Option<Value> {
        let mut worker = self.worker.borrow_mut();
        let mut scope = realm.handle_scope(worker.js_runtime.v8_isolate());
        let result = convert_v8_to_ruby(value, &mut scope);

        match result {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }

    pub(crate) fn to_v8(
        &self,
        realm: &JsRealm,
        value: Value,
    ) -> Option<Global<v8::Value>> {
        let mut worker = self.worker.borrow_mut();
        let mut scope = realm.handle_scope(worker.js_runtime.v8_isolate());
        let value = convert_ruby_to_v8(value, &mut scope).unwrap();
        let value = Global::<v8::Value>::new(&mut scope, value);

        Some(value)
    }

    pub(crate) fn send<'a>(
        &self,
        scope: &mut HandleScope,
        value: serde_v8::Value<'a>,
    ) -> Result<serde_v8::Value<'a>, AnyError> {
        if self.ruby_receiver.borrow_mut().as_mut().is_none() {
            return Err(AnyError::msg(
                "Cannot send to ruby. The ruby receiver is missing, please \
                initialize and set one before calling into Ruby?",
            ));
        }

        // we can send with and without gvl, if no ruby context is present, we
        // assume that GVL is held
        if let (None, Some(rec)) = (
            self.ruby_context.borrow_mut().as_mut(),
            self.ruby_receiver.borrow_mut().as_mut(),
        ) {
            let value = value.v8_value;
            let value = Global::<v8::Value>::new(scope, value);
            let value = convert_v8_to_ruby(&value, scope).unwrap();
            let args: (Value,) = (value,);
            return rec
                .call::<(Value,), Value>(args)
                .map_err(|err| AnyError::msg(format!("{:?}", err)))
                .and_then(|value| convert_ruby_to_v8(value, scope))
                .map(|value| from_v8(scope, value).unwrap());
        }

        // we need to deref the receiver as mut, as it is behind an Option
        // TODO: make sure all operations on Ruby data happen when GVL is held
        if let Some(ctx) = self.ruby_context.borrow_mut().as_mut() {
            ctx.with_gvl(|| {
                if let Some(rec) = self.ruby_receiver.borrow_mut().as_mut() {
                    let value = value.v8_value;
                    let value = Global::<v8::Value>::new(scope, value);
                    let value = convert_v8_to_ruby(&value, scope)?;
                    let args: (Value,) = (value,);
                    rec.call::<(Value,), Value>(args)
                        .map_err(|err| AnyError::msg(format!("{:?}", err)))
                } else {
                    Err(AnyError::msg("cannot access ruby receiver"))
                }
            })
            .map_err(|err| AnyError::msg(format!("{:?}", err)))?
            .and_then(|value| convert_ruby_to_v8(value, scope))
            .map(|value| from_v8(scope, value).unwrap())
        } else {
            Err(AnyError::msg("this should never happen"))
        }
    }
}

impl Default for Worker {
    fn default() -> Self {
        let module_loader = Rc::new(FsModuleLoader);
        let create_web_worker_cb = Arc::new(|_| {
            todo!("Web workers are not supported in the example");
        });

        let ext_isorun = ext::isorun::send_to_ruby::init_ops();
        let mut extensions = vec![ext_isorun];

        let options = WorkerOptions {
            bootstrap: BootstrapOptions {
                args: vec![],
                cpu_count: 1,
                log_level: Default::default(),
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
                has_node_modules_dir: false,
                maybe_binary_npm_command_name: None,
            },
            extensions: std::mem::take(&mut extensions),
            startup_snapshot: None,
            create_params: None,
            unsafely_ignore_certificate_errors: None,
            seed: None,
            source_map_getter: None,
            format_js_error_fn: None,
            create_web_worker_cb,
            maybe_inspector_server: None,
            should_break_on_first_statement: false,
            module_loader,
            npm_resolver: None,
            get_error_class_fn: Some(&get_error_class_name),
            cache_storage_dir: None,
            origin_storage_dir: None,
            blob_store: Arc::from(BlobStore::default()),
            broadcast_channel: InMemoryBroadcastChannel::default(),
            shared_array_buffer_store: None,
            compiled_wasm_module_store: None,
            stdio: Default::default(),
            should_wait_for_inspector_session: false,
            root_cert_store_provider: None,
            fs: Arc::new(RealFs),
        };

        // todo: we don't use the main module at all, but it could be used as an
        //  entry point for "eval" JavaScript.
        let isorun_native_gem_path =
            env::var("ISORUN_NATIVE_GEM_PATH").unwrap();
        let js_path = Path::new(isorun_native_gem_path.as_str())
            .join("ext/isorun/src/call.js");

        let main_module = deno_core::resolve_path(
            &js_path.to_string_lossy(),
            &env::current_dir().context("Unable to get CWD").unwrap(),
        )
        .unwrap();
        let permissions = PermissionsContainer::allow_all();
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
