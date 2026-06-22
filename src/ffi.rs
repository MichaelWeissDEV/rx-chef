/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * License:     Apache-2.0
 * Description: rxchef backend FFI
 * -----------------------------------------------------------------------------
 */

use std::{
    ffi::{CStr, CString},
    os::raw::{c_char, c_uchar},
    ptr, slice,
};

use crate::{
    operation::ArgValue,
    operations::{get_operation, operation_names},
    runtime,
};

/**
 * @struct RxChefResult
 * @brief Represents the result from an rxchef operation.
 */
#[repr(C)]
pub struct RxChefResult {
    pub data: *mut c_uchar,
    pub length: usize,
    pub capacity: usize,
    pub error: *mut c_char,
}

#[derive(serde::Serialize)]
struct OpMetadata {
    name: String,
    module: String,
    description: String,
    args: Vec<ArgMetadata>,
}

#[derive(serde::Serialize)]
struct ArgMetadata {
    name: String,
    description: String,
    default_value: String,
}

/**
 * @brief Lists all available operations separated by semicolon.
 * @return Semicolon-separated string of operation names. Caller must free via rxchef_free_string.
 */
#[no_mangle]
pub extern "C" fn rxchef_list_operations() -> *mut c_char {
    let names = operation_names();
    let joined = names.join(";");
    CString::new(joined).unwrap_or_default().into_raw()
}

/// Returns JSON metadata for an operation.
///
/// # Safety
/// The caller must ensure op_name is a valid null-terminated C string.
#[no_mangle]
pub unsafe extern "C" fn rxchef_get_metadata(op_name: *const c_char) -> *mut c_char {
    if op_name.is_null() {
        return ptr::null_mut();
    }
    let name = CStr::from_ptr(op_name).to_string_lossy();
    let canonical = runtime::resolve_operation_name(&name);
    let op = match canonical.as_deref().and_then(|n| get_operation(n)) {
        Some(o) => o,
        None => return ptr::null_mut(),
    };

    let metadata = OpMetadata {
        name: op.name().to_string(),
        module: op.module().to_string(),
        description: op.description().to_string(),
        args: op
            .args_schema()
            .iter()
            .map(|a| ArgMetadata {
                name: a.name.to_string(),
                description: a.description.to_string(),
                default_value: a.default_value.to_string(),
            })
            .collect(),
    };

    let json = serde_json::to_string(&metadata).unwrap_or_default();
    CString::new(json).unwrap_or_default().into_raw()
}

/**
 * @brief Returns JSON array of metadata for all operations.
 * @return JSON string of metadata array. Caller must free via rxchef_free_string.
 */
#[no_mangle]
pub extern "C" fn rxchef_get_all_metadata() -> *mut c_char {
    let names = operation_names();
    let mut all_meta = Vec::with_capacity(names.len());

    for name in names {
        if let Some(op) = get_operation(&name) {
            all_meta.push(OpMetadata {
                name: op.name().to_string(),
                module: op.module().to_string(),
                description: op.description().to_string(),
                args: op
                    .args_schema()
                    .iter()
                    .map(|a| ArgMetadata {
                        name: a.name.to_string(),
                        description: a.description.to_string(),
                        default_value: a.default_value.to_string(),
                    })
                    .collect(),
            });
        }
    }

    let json = serde_json::to_string(&all_meta).unwrap_or_default();
    CString::new(json).unwrap_or_default().into_raw()
}

/// Analyzes input to suggest operations.
///
/// # Safety
/// The caller must ensure input_data is valid for input_len bytes.
#[no_mangle]
pub unsafe extern "C" fn rxchef_magic(input_data: *const c_uchar, input_len: usize) -> *mut c_char {
    let input = if input_len > 0 && !input_data.is_null() {
        slice::from_raw_parts(input_data, input_len)
    } else {
        &[]
    };
    let results = crate::magic::analyze_input(input);
    let json = serde_json::to_string(&results).unwrap_or_default();
    CString::new(json).unwrap_or_default().into_raw()
}

/// Frees a string allocated by Rust.
///
/// # Safety
/// The caller must ensure s was allocated by Rust and not already freed.
#[no_mangle]
pub unsafe extern "C" fn rxchef_free_string(s: *mut c_char) {
    if !s.is_null() {
        let _ = CString::from_raw(s);
    }
}

/// Creates a string argument for rxchef.
///
/// # Safety
/// The caller must ensure s is a valid null-terminated C string.
#[no_mangle]
pub unsafe extern "C" fn rxchef_arg_str(s: *const c_char) -> *mut ArgValue {
    if s.is_null() {
        return ptr::null_mut();
    }
    let c_str = CStr::from_ptr(s);
    let string = c_str.to_string_lossy().into_owned();
    Box::into_raw(Box::new(ArgValue::Str(string)))
}

/**
 * @brief Creates a numeric argument for rxchef.
 * @param n The numeric value.
 * @return Pointer to ArgValue.
 */
#[no_mangle]
pub extern "C" fn rxchef_arg_num(n: f64) -> *mut ArgValue {
    Box::into_raw(Box::new(ArgValue::Num(n)))
}

/**
 * @brief Creates a boolean argument for rxchef.
 * @param b The boolean value.
 * @return Pointer to ArgValue.
 */
#[no_mangle]
pub extern "C" fn rxchef_arg_bool(b: bool) -> *mut ArgValue {
    Box::into_raw(Box::new(ArgValue::Bool(b)))
}

/// Creates a byte array argument for rxchef.
///
/// # Safety
/// The caller must ensure data is valid for length bytes.
#[no_mangle]
pub unsafe extern "C" fn rxchef_arg_bytes(data: *const c_uchar, length: usize) -> *mut ArgValue {
    if data.is_null() && length > 0 {
        return ptr::null_mut();
    }
    let slice = if length > 0 {
        slice::from_raw_parts(data, length)
    } else {
        &[]
    };
    Box::into_raw(Box::new(ArgValue::Bytes(slice.to_vec())))
}

/// Frees an ArgValue.
///
/// # Safety
/// The caller must ensure arg was allocated by rxchef_arg_* functions.
#[no_mangle]
pub unsafe extern "C" fn rxchef_free_arg(arg: *mut ArgValue) {
    if !arg.is_null() {
        let _ = Box::from_raw(arg);
    }
}

/// Executes an rxchef operation.
///
/// # Safety
/// The caller must ensure all pointers are valid and arguments match the operation schema.
#[no_mangle]
pub unsafe extern "C" fn rxchef_run(
    op_name: *const c_char,
    input_data: *const c_uchar,
    input_len: usize,
    args: *const *mut ArgValue,
    num_args: usize,
) -> *mut RxChefResult {
    if op_name.is_null() {
        return ptr::null_mut();
    }

    let name = CStr::from_ptr(op_name).to_string_lossy();
    let canonical = runtime::resolve_operation_name(&name);
    let op = match canonical.as_deref().and_then(|n| get_operation(n)) {
        Some(o) => o,
        None => {
            let res = Box::new(RxChefResult {
                data: ptr::null_mut(),
                length: 0,
                capacity: 0,
                error: CString::new(format!("Operation '{}' not found", name))
                    .unwrap_or_default()
                    .into_raw(),
            });
            return Box::into_raw(res);
        }
    };

    let input = if input_len > 0 && !input_data.is_null() {
        slice::from_raw_parts(input_data, input_len).to_vec()
    } else {
        Vec::new()
    };

    let mut rust_args = Vec::with_capacity(num_args);
    if !args.is_null() && num_args > 0 {
        let args_slice = slice::from_raw_parts(args, num_args);
        for &arg_ptr in args_slice {
            if !arg_ptr.is_null() {
                rust_args.push((*arg_ptr).clone());
            } else {
                rust_args.push(ArgValue::Str(String::new()));
            }
        }
    }

    let run_result =
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| op.run(input, &rust_args)));
    let (out_data, out_len, out_cap, out_err) = match run_result {
        Ok(Ok(mut v)) => {
            let len = v.len();
            let cap = v.capacity();
            let ptr = v.as_mut_ptr();
            std::mem::forget(v);
            (ptr, len, cap, ptr::null_mut())
        }
        Ok(Err(e)) => {
            let err_str = CString::new(e.to_string()).unwrap_or_default();
            (ptr::null_mut(), 0, 0, err_str.into_raw())
        }
        Err(_) => {
            let err_str = CString::new("operation panicked").unwrap_or_default();
            (ptr::null_mut(), 0, 0, err_str.into_raw())
        }
    };

    let result = Box::new(RxChefResult {
        data: out_data,
        length: out_len,
        capacity: out_cap,
        error: out_err,
    });
    Box::into_raw(result)
}

/// Frees an RxChefResult.
///
/// # Safety
/// The caller must ensure res was allocated by rxchef_run.
#[no_mangle]
pub unsafe extern "C" fn rxchef_free_result(res: *mut RxChefResult) {
    if res.is_null() {
        return;
    }
    let result = Box::from_raw(res);
    if !result.data.is_null() {
        let _ = Vec::from_raw_parts(result.data, result.length, result.capacity);
    }
    if !result.error.is_null() {
        let _ = CString::from_raw(result.error);
    }
}
