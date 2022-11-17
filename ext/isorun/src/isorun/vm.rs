use deno_core::error::AnyError;
use deno_core::{Extension, FsModuleLoader, ModuleId};
use deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_runtime::permissions::Permissions;
use deno_runtime::worker::{MainWorker, WorkerOptions};
use deno_runtime::BootstrapOptions;
use deno_web::BlobStore;
use std::borrow::BorrowMut;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use v8::{Global, Local, Value};

fn get_error_class_name(e: &AnyError) -> &'static str {
    deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
}

pub struct VM {
    worker: MainWorker,
    module_id: ModuleId,
}

impl VM {
    pub async fn new(extensions: Vec<Extension>) -> VM {
        let (worker, module_id) =
            VM::preload(extensions).await.expect("cannot preload app");

        VM { worker, module_id }
    }

    pub(crate) async fn render(
        &mut self,
        bundle_path: &str,
    ) -> Result<String, AnyError> {
        let module_namespace;
        {
            let js_runtime = self.worker.borrow_mut().js_runtime.borrow_mut();
            module_namespace =
                js_runtime.get_module_namespace(self.module_id).unwrap();
        }

        let promise: Global<Value>;
        {
            let js_runtime = self.worker.borrow_mut().js_runtime.borrow_mut();
            let mut scope = js_runtime
                .create_realm()
                .unwrap()
                .handle_scope(js_runtime.v8_isolate());

            let module_namespace =
                Local::<v8::Object>::new(&mut scope, module_namespace);
            let export_name = v8::String::new(&mut scope, "render").unwrap();
            let binding = module_namespace
                .get(&mut scope, export_name.into())
                .unwrap();
            let func = v8::Local::<v8::Function>::try_from(binding)
                .expect("cannot extract function");

            // call function
            let path_arg = v8::String::new(&mut scope, bundle_path).unwrap();
            let args: &[Local<Value>] = &[path_arg.into()];
            let recv = v8::undefined(scope.borrow_mut());
            let maybe_result =
                func.call(scope.as_mut(), recv.into(), args).unwrap();
            promise = Global::new(&mut scope, maybe_result);
        }

        let value;
        {
            let js_runtime = self.worker.borrow_mut().js_runtime.borrow_mut();
            js_runtime.run_event_loop(false).await?;
            value = js_runtime.resolve_value(promise).await?;
        }

        let html;
        {
            let js_runtime = self.worker.borrow_mut().js_runtime.borrow_mut();
            let scope = &mut js_runtime.handle_scope();
            html = value.open(scope).to_rust_string_lossy(scope);
        }

        Ok(html)
    }

    async fn preload(
        mut extensions: Vec<Extension>,
    ) -> Result<(MainWorker, ModuleId), AnyError> {
        let module_loader = Rc::new(FsModuleLoader);
        let create_web_worker_cb = Arc::new(|_| {
            todo!("Web workers are not supported in the example");
        });
        let web_worker_event_cb = Arc::new(|_| {
            todo!("Web workers are not supported in the example");
        });

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
                user_agent: "isorun".to_string(),
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

        let main_app_path = "/Users/hannesmoser/src/github.com/eliias/isorun/ext/isorun/src/render.js";
        let js_app_path = Path::new(main_app_path);
        let main_module =
            deno_core::resolve_path(&js_app_path.to_string_lossy())?;
        let permissions = Permissions::allow_all();

        let mut worker = MainWorker::bootstrap_from_options(
            main_module.clone(),
            permissions,
            options,
        );
        let module_id = worker.preload_main_module(&main_module).await.unwrap();
        worker.evaluate_module(module_id).await?;
        worker.run_event_loop(false).await?;

        Ok((worker, module_id))
    }
}
