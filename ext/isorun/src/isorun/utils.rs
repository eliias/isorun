use deno_core::error::AnyError;
use magnus::r_hash::ForEach;
use magnus::value::{Qfalse, Qtrue};
use magnus::{
    Integer, RArray, RFloat, RHash, RString, RStruct, Symbol, Value, QFALSE,
    QNIL, QTRUE,
};
use v8::{Array, GetPropertyNamesArgs, HandleScope, Local, Object};

pub fn convert_v8_to_ruby(
    value: Local<v8::Value>,
    scope: &mut HandleScope,
) -> Result<Value, AnyError> {
    if value.is_null() {
        return Ok(Value::from(QNIL));
    }

    if value.is_int32() {
        return Ok(Value::from(Integer::from_i64(
            value.int32_value(scope).unwrap() as i64,
        )));
    }

    if value.is_number() {
        return Ok(Value::from(
            RFloat::from_f64(value.number_value(scope).unwrap()).unwrap(),
        ));
    }

    if value.is_true() {
        return Ok(Value::from(QTRUE));
    }

    if value.is_false() {
        return Ok(Value::from(QFALSE));
    }

    if value.is_string() {
        let raw = value.to_rust_string_lossy(scope);
        return Ok(Value::from(RString::from(raw)));
    }

    if value.is_array() {
        let arr = Local::<Array>::try_from(value).unwrap();
        let length = arr.length();
        let r_arr = RArray::with_capacity(length as usize);
        for i in 0..length {
            let raw = arr.get_index(scope, i).unwrap();
            let val = convert_v8_to_ruby(raw, scope).unwrap();
            r_arr.push(val).expect("cannot add item to array");
        }
        return Ok(Value::from(r_arr));
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
            let raw_val = obj.get(scope, raw_key).unwrap();
            let key = convert_v8_to_ruby(raw_key, scope).unwrap();
            let val = convert_v8_to_ruby(raw_val, scope).unwrap();
            r_hash.aset(key, val).expect("cannot set item to hash");
        }
        return Ok(Value::from(r_hash));
    }

    Ok(Value::from(QNIL))
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

        for (i, val) in v.each().enumerate() {
            let v8_value;
            {
                v8_value = convert_ruby_to_v8(val.unwrap(), scope).unwrap();
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
