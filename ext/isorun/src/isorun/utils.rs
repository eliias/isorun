use crate::js::worker::WORKER;
use deno_core::error::AnyError;
use magnus::r_hash::ForEach;
use magnus::value::{qfalse, qnil, qtrue, Qfalse, Qtrue, ReprValue};
use magnus::{Integer, RArray, RFloat, RHash, RString, RStruct, Symbol, Value};
use std::collections::HashMap;
use v8::{Array, GetPropertyNamesArgs, Global, HandleScope, Local, Object};

pub fn convert_v8_to_ruby(
    value: &Global<v8::Value>,
    scope: &mut HandleScope,
) -> Result<Value, AnyError> {
    let value = Local::new(scope, value);
    if value.is_null() {
        return Ok(qnil().as_value());
    }

    if value.is_int32() {
        return Ok(Integer::from_i64(value.int32_value(scope).unwrap() as i64)
            .as_value());
    }

    if value.is_number() {
        return Ok(RFloat::from_f64(value.number_value(scope).unwrap())
            .unwrap()
            .as_value());
    }

    if value.is_true() {
        return Ok(qtrue().as_value());
    }

    if value.is_false() {
        return Ok(qfalse().as_value());
    }

    if value.is_string() {
        let str = value.to_rust_string_lossy(scope);
        return Ok(RString::new(str.as_str()).as_value());
    }

    if value.is_array() {
        let arr = Local::<Array>::try_from(value).unwrap();
        let length = arr.length();
        let r_arr = RArray::with_capacity(length as usize);
        for i in 0..length {
            let raw = arr.get_index(scope, i).unwrap();
            let global_raw = Global::<v8::Value>::new(scope, raw);
            let val = convert_v8_to_ruby(&global_raw, scope).unwrap();
            r_arr.push(val).expect("cannot add item to array");
        }
        return Ok(r_arr.as_value());
    }

    if value.is_object() {
        let obj = Local::<Object>::try_from(value).unwrap();
        let properties = obj
            .get_own_property_names(scope, GetPropertyNamesArgs::default())
            .unwrap();
        let length = properties.length();
        let r_hash = RHash::new();
        for i in 0..length {
            let raw_key = properties.get_index(scope, i).unwrap();
            let global_raw_key = Global::<v8::Value>::new(scope, raw_key);
            let raw_val = obj.get(scope, raw_key).unwrap();
            let global_raw_val = Global::<v8::Value>::new(scope, raw_val);
            let key = convert_v8_to_ruby(&global_raw_key, scope).unwrap();
            let val = convert_v8_to_ruby(&global_raw_val, scope).unwrap();
            r_hash.aset(key, val).expect("cannot set item to hash");
        }
        return Ok(r_hash.as_value());
    }

    Ok(qnil().as_value())
}

pub fn convert_ruby_to_v8<'s>(
    value: Value,
    scope: &mut HandleScope<'s>,
) -> Result<Local<'s, v8::Value>, AnyError> {
    if value.is_nil() {
        return Ok(v8::null(scope).into());
    }

    if let Some(v) = Qtrue::from_value(value) {
        return Ok(v8::Boolean::new(scope, v.to_bool()).into());
    }

    if let Some(v) = Qfalse::from_value(value) {
        return Ok(v8::Boolean::new(scope, v.to_bool()).into());
    }

    if let Some(v) = Symbol::from_value(value) {
        return Ok(v8::String::new(scope, v.to_string().as_str())
            .unwrap()
            .into());
    }

    if let Some(v) = Integer::from_value(value) {
        return Ok(v8::Integer::new(scope, v.to_i32().unwrap()).into());
    }

    if let Some(v) = RFloat::from_value(value) {
        return Ok(v8::Number::new(scope, v.to_f64()).into());
    }

    if let Some(v) = RString::from_value(value) {
        return Ok(v8::String::new(scope, v.to_string().unwrap().as_str())
            .unwrap()
            .into());
    }

    if let Some(v) = RArray::from_value(value) {
        let arr;
        {
            arr = Array::new(scope, v.len() as i32);
        }

        for (i, val) in v.into_iter().enumerate() {
            let v8_value;
            {
                v8_value = convert_ruby_to_v8(val, scope).unwrap();
            }
            arr.set_index(scope, i as u32, v8_value);
        }
        return Ok(arr.into());
    }

    if let Some(v) = RHash::from_value(value) {
        let obj = Object::new(scope);
        v.foreach(|key: Value, val: Value| {
            let key = convert_ruby_to_v8(key, scope).unwrap();
            let val = convert_ruby_to_v8(val, scope).unwrap();
            obj.set(scope, key, val);

            Ok(ForEach::Continue)
        })
        .expect("cannot convert hash into JavaScript object");

        return Ok(obj.into());
    }

    if let Some(v) = RStruct::from_value(value) {
        let obj = Object::new(scope);
        for member in v.members().unwrap() {
            let key = member.to_string();
            let val = v.getmember::<&str, Value>(key.as_str()).unwrap();
            let v8_key = v8::String::new(scope, key.as_str()).unwrap();
            let v8_val = convert_ruby_to_v8(val, scope).unwrap();

            obj.set(scope, v8_key.into(), v8_val);
        }

        return Ok(obj.into());
    }

    Ok(v8::null(scope).into())
}

pub(crate) fn low_memory_notification() {
    WORKER.with(|worker| {
        let mut worker = worker.worker.borrow_mut();
        worker.js_runtime.v8_isolate().low_memory_notification();
    });
}

pub(crate) fn stats() -> RHash {
    WORKER.with(|worker| {
        let mut worker = worker.worker.borrow_mut();

        let heap_stats = &mut Default::default();
        worker
            .js_runtime
            .v8_isolate()
            .get_heap_statistics(heap_stats);

        let current_thread = std::thread::current();
        let thread = HashMap::from([(
            "thread_id",
            format!("{:?}", current_thread.id()),
        )]);

        let heap = HashMap::from([
            ("external_memory", heap_stats.external_memory()),
            ("heap_size_limit", heap_stats.heap_size_limit()),
            ("malloced_memory", heap_stats.malloced_memory()),
            (
                "number_of_detached_contexts",
                heap_stats.number_of_detached_contexts(),
            ),
            (
                "number_of_native_contexts",
                heap_stats.number_of_native_contexts(),
            ),
            ("peak_malloced_memory", heap_stats.peak_malloced_memory()),
            ("total_available_size", heap_stats.total_available_size()),
            (
                "total_global_handles_size",
                heap_stats.total_global_handles_size(),
            ),
            ("total_heap_size", heap_stats.total_heap_size()),
            (
                "total_heap_size_executable",
                heap_stats.total_heap_size_executable(),
            ),
            ("total_physical_size", heap_stats.total_physical_size()),
            (
                "used_global_handles_size",
                heap_stats.used_global_handles_size(),
            ),
            ("used_heap_size", heap_stats.used_heap_size()),
        ]);

        let h = RHash::new();
        h.aset("current_thread", RHash::from_iter(thread))
            .expect("cannot set stats for current_thread");
        h.aset("heap", RHash::from_iter(heap))
            .expect("cannot set stats for heap");

        h
    })
}
