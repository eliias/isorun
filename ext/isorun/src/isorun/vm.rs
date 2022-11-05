use std::borrow::{BorrowMut};
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use deno_core::{Extension, FsModuleLoader, ModuleId};
use deno_core::error::{AnyError};
use deno_runtime::{BootstrapOptions};
use deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_runtime::permissions::Permissions;
use deno_runtime::worker::{MainWorker, WorkerOptions};
use deno_web::BlobStore;
use magnus::{Error};
use v8::{Local};

fn get_error_class_name(e: &AnyError) -> &'static str {
    deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
}

pub struct VM;

impl VM {
    pub fn new() -> VM { VM {} }

    async fn _call(&self, mut worker: MainWorker, module_id: ModuleId) -> Result<String, AnyError> {
        let module_namespace;
        {
            let js_runtime = worker.js_runtime.borrow_mut();
            module_namespace = js_runtime.get_module_namespace(module_id).unwrap();
        }

        let promise;
        {
            let js_runtime = worker.js_runtime.borrow_mut();
            let mut scope = js_runtime.handle_scope();

            let module_namespace =
                Local::<v8::Object>::new(&mut scope, module_namespace);
            let export_name = v8::String::new(&mut scope, "render").unwrap();
            let binding = module_namespace.get(&mut scope, export_name.into()).unwrap();
            let func = v8::Local::<v8::Function>::try_from(binding)?;

            // call function
            let resource_id = v8::Number::new(scope.as_mut(), 1 as f64);
            let resource_id_value = Local::from(resource_id);

            let args = &[resource_id_value];
            let result = func.call(&mut scope, module_namespace.into(), args).unwrap();
            promise = v8::Global::new(&mut scope, result);
        }

        let value;
        {
            let js_runtime = worker.js_runtime.borrow_mut();
            js_runtime.run_event_loop(false).await?;
            value = js_runtime.resolve_value(promise).await?;
        }

        let html;
        {
            let js_runtime = worker.js_runtime.borrow_mut();
            let scope = &mut js_runtime.handle_scope();
            html = value.open(scope).to_rust_string_lossy(scope);
        }

        Ok(html)
    }

    async fn _render(&self, app_path: String, mut extensions: Vec<Extension>) -> Result<String, AnyError> {
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
                user_agent: "hello_runtime".to_string(),
                inspect: false,
            },
            extensions: std::mem::take(&mut extensions),
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

        let js_app_path = Path::new(&app_path);
        let main_module = deno_core::resolve_path(&js_app_path.to_string_lossy())?;
        let permissions = Permissions::allow_all();

        let mut worker = MainWorker::bootstrap_from_options(
            main_module.clone(),
            permissions,
            options,
        );
        let module_id = worker.preload_main_module(&main_module).await.unwrap();
        worker.evaluate_module(module_id).await?;
        worker.run_event_loop(false).await?;

        // call render function
        let html = self._call(worker, module_id).await?;

        Ok(html)
    }

    pub fn render(&self, app_path: String, extensions: Vec<Extension>) -> Result<String, Error> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        runtime.block_on(self._render(app_path.clone(), extensions))
            .map_err(|error| Error::runtime_error(
                format!("cannot render app: {}, error: {}", app_path, error))
            )
    }
}
