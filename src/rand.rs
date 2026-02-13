#![allow(dead_code)]
use napi::bindgen_prelude::{Either, Object, Unknown};
use napi_derive::napi;

const DEFAULT_MIN_I64: i64 = 0;
const DEFAULT_MAX_I64: i64 = 5;
const DEFAULT_MIN_F64: f64 = 0.0;
const DEFAULT_MAX_F64: f64 = 5.0;
const DEFAULT_STR_LENGTH: usize = 5;

const LETTERS_CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const ALL_CHARSET: &[u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+-=[]{}|;:,.<>?/`~";

#[inline(always)]
fn random_int_range(a: i64, b: i64) -> i64 {
    let lo = a.min(b);
    let hi = a.max(b);
    fastrand::i64(lo..=hi)
}

#[inline(always)]
fn random_float_range(a: f64, b: f64) -> f64 {
    let lo = a.min(b);
    let hi = a.max(b);
    if lo == hi {
        return lo;
    }
    fastrand::f64() * (hi - lo) + lo
}

#[inline(always)]
fn random_index(len: usize) -> usize {
    fastrand::usize(..len)
}

#[inline(always)]
fn resolve_bounds<T: Copy>(
    min: Option<T>,
    max: Option<T>,
    default_min: T,
    default_max: T,
    single_arg_min: T,
) -> (T, T) {
    match (min, max) {
        (None, None) => (default_min, default_max),
        (Some(value), None) | (None, Some(value)) => (single_arg_min, value),
        (Some(min_value), Some(max_value)) => (min_value, max_value),
    }
}

#[inline(always)]
fn string_config(
    length_or_letters: Option<Either<u32, bool>>,
    letters: Option<bool>,
) -> (usize, bool) {
    match length_or_letters {
        Some(Either::A(length)) => (length as usize, letters.unwrap_or(false)),
        Some(Either::B(letters_only)) => (DEFAULT_STR_LENGTH, letters_only),
        None => (DEFAULT_STR_LENGTH, letters.unwrap_or(false)),
    }
}

#[inline(always)]
fn random_charset(letters_only: bool) -> &'static [u8] {
    if letters_only {
        return LETTERS_CHARSET;
    }
    ALL_CHARSET
}

#[inline(always)]
fn random_string_from_charset(length: usize, charset: &[u8]) -> String {
    let mut output = String::with_capacity(length);
    for _ in 0..length {
        let index = random_index(charset.len());
        output.push(charset[index] as char);
    }
    output
}

#[inline(always)]
fn object_keys(values: &Object) -> napi::Result<Object> {
    values.get_property_names()
}

#[inline(always)]
fn object_mode(
    values: &Object,
    key_or_keys: Option<Either<bool, Object>>,
    return_key: Option<bool>,
) -> napi::Result<(Object, bool)> {
    let default_return_key = return_key.unwrap_or(false);
    let Some(mode) = key_or_keys else {
        return Ok((object_keys(values)?, default_return_key));
    };

    match mode {
        Either::A(flag) => Ok((object_keys(values)?, flag)),
        Either::B(keys) => Ok((keys, default_return_key)),
    }
}

#[inline(always)]
fn random_object_element(values: &Object) -> napi::Result<Option<Unknown>> {
    let length = values.get_array_length()?;
    if length == 0 {
        return Ok(None);
    }

    let index = random_index(length as usize) as u32;
    let value: Unknown = values.get_element_unchecked(index)?;
    Ok(Some(value))
}

#[inline(always)]
fn random_object_key(keys: &Object) -> napi::Result<Option<Unknown>> {
    random_object_element(keys)
}

#[inline(always)]
fn random_array_value(values: &Object) -> napi::Result<Option<Unknown>> {
    random_object_element(values)
}

#[napi(namespace = "rand")]
pub fn int(min: Option<i64>, max: Option<i64>) -> i64 {
    let (a, b) = resolve_bounds(min, max, DEFAULT_MIN_I64, DEFAULT_MAX_I64, 1);
    random_int_range(a, b)
}

#[napi(namespace = "rand")]
pub fn float(min: Option<f64>, max: Option<f64>) -> f64 {
    let (a, b) = resolve_bounds(min, max, DEFAULT_MIN_F64, DEFAULT_MAX_F64, 1.0);
    random_float_range(a, b)
}

#[napi(js_name = "str", namespace = "rand")]
pub fn random_string(
    length_or_letters: Option<Either<u32, bool>>,
    letters: Option<bool>,
) -> String {
    let (length, letters_only) = string_config(length_or_letters, letters);
    if length == 0 {
        return String::new();
    }

    let charset = random_charset(letters_only);
    random_string_from_charset(length, charset)
}

#[napi(js_name = "bool", namespace = "rand")]
pub fn random_bool() -> bool {
    fastrand::bool()
}

#[napi(namespace = "rand")]
pub fn array(values: Object) -> napi::Result<Option<Unknown>> {
    random_array_value(&values)
}

#[napi(namespace = "rand")]
pub fn object(
    values: Object,
    key_or_keys: Option<Either<bool, Object>>,
    return_key: Option<bool>,
) -> napi::Result<Option<Unknown>> {
    let (keys, should_return_key) = object_mode(&values, key_or_keys, return_key)?;
    let selected_key = random_object_key(&keys)?;
    let Some(selected_key) = selected_key else {
        return Ok(None);
    };

    if should_return_key {
        return Ok(Some(selected_key));
    }

    let selected_value: Unknown = values.get_property_unchecked(selected_key)?;
    Ok(Some(selected_value))
}
