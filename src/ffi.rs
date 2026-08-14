/*
 * -----------------------------------------------------------------------------
 * Project:     rxchef
 * Author:      Michael Weiss
 * License:     Apache-2.0
 * Description: Experimental rxchef backend FFI (ABI stability is not guaranteed)
 * -----------------------------------------------------------------------------
 */

use std::{
    ffi::{CStr, CString},
    os::raw::{c_char, c_uchar},
    ptr, slice,
};

use crate::{
    execution,
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

fn error_result(message: impl Into<String>) -> *mut RxChefResult {
    let message = message.into().replace('\0', "\\0");
    Box::into_raw(Box::new(RxChefResult {
        data: ptr::null_mut(),
        length: 0,
        capacity: 0,
        error: CString::new(message).unwrap_or_default().into_raw(),
    }))
}

fn ffi_boundary<T>(operation: impl FnOnce() -> T) -> Result<T, ()> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(operation)).map_err(|_| ())
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
    let results = crate::magic::magic(input, &crate::magic::MagicOptions::default());
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
    if !n.is_finite() {
        return ptr::null_mut();
    }
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
        return error_result("op_name must not be NULL");
    }
    if input_data.is_null() && input_len > 0 {
        return error_result("input_data must not be NULL when input_len is non-zero");
    }
    if args.is_null() && num_args > 0 {
        return error_result("args must not be NULL when num_args is non-zero");
    }

    let name = match CStr::from_ptr(op_name).to_str() {
        Ok(name) => name,
        Err(_) => return error_result("op_name must be valid UTF-8"),
    };
    let input_supplied = !input_data.is_null();
    let input = if input_len > 0 {
        slice::from_raw_parts(input_data, input_len).to_vec()
    } else {
        Vec::new()
    };

    let mut raw_args = Vec::with_capacity(num_args);
    if num_args > 0 {
        let args_slice = slice::from_raw_parts(args, num_args);
        for &arg_ptr in args_slice {
            if arg_ptr.is_null() {
                return error_result("argument pointers must not be NULL");
            }
            raw_args.push(match &*arg_ptr {
                ArgValue::Str(value) => value.clone(),
                ArgValue::Num(value) => format!("num:{value}"),
                ArgValue::Bool(value) => format!("bool:{value}"),
                ArgValue::Bytes(value) => format!("hex:{}", hex::encode(value)),
            });
        }
    }

    let run_result = ffi_boundary(|| {
        execution::execute(execution::ExecutionRequest {
            input,
            input_supplied,
            recipe: vec![execution::RecipeStep {
                op: name.to_string(),
                args: raw_args,
            }]
            .into(),
            variables: execution::VariableContext::default(),
            options: execution::ExecutionOptions::default(),
        })
        .map(|outcome| outcome.output)
    });
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

#[cfg(test)]
mod tests {
    use super::*;

    unsafe fn error_text(result: *mut RxChefResult) -> String {
        assert!(!result.is_null(), "FFI returned a NULL result object");
        assert!(
            !(*result).error.is_null(),
            "FFI result unexpectedly succeeded"
        );
        let text = CStr::from_ptr((*result).error)
            .to_string_lossy()
            .into_owned();
        rxchef_free_result(result);
        text
    }

    #[test]
    fn run_rejects_inconsistent_null_pointer_lengths() {
        let operation = CString::new("To Base64").unwrap();
        let result = unsafe { rxchef_run(operation.as_ptr(), ptr::null(), 1, ptr::null(), 0) };
        assert!(unsafe { error_text(result) }.contains("input_data"));

        let result = unsafe { rxchef_run(operation.as_ptr(), ptr::null(), 0, ptr::null(), 1) };
        assert!(unsafe { error_text(result) }.contains("args"));
    }

    #[test]
    fn run_returns_binary_with_explicit_ownership() {
        let operation = CString::new("From Base64").unwrap();
        let input = b"AAH/";
        let result = unsafe {
            rxchef_run(
                operation.as_ptr(),
                input.as_ptr(),
                input.len(),
                ptr::null(),
                0,
            )
        };
        assert!(!result.is_null());
        assert!(unsafe { (*result).error.is_null() });
        assert_eq!(
            unsafe { slice::from_raw_parts((*result).data, (*result).length) },
            [0, 1, 255]
        );
        unsafe { rxchef_free_result(result) };
    }

    #[test]
    fn run_distinguishes_missing_from_explicit_empty_input() {
        let operation = CString::new("To Base64").unwrap();
        let missing = unsafe { rxchef_run(operation.as_ptr(), ptr::null(), 0, ptr::null(), 0) };
        assert!(unsafe { error_text(missing) }.contains("input source is required"));

        let sentinel = [0_u8; 1];
        let empty = unsafe { rxchef_run(operation.as_ptr(), sentinel.as_ptr(), 0, ptr::null(), 0) };
        assert!(unsafe { (*empty).error.is_null() });
        assert_eq!(unsafe { (*empty).length }, 0);
        unsafe { rxchef_free_result(empty) };
    }

    #[test]
    fn run_reports_unknown_operation_invalid_arguments_and_operation_errors() {
        let unknown = CString::new("Definitely Missing").unwrap();
        let input = b"x";
        let result = unsafe {
            rxchef_run(
                unknown.as_ptr(),
                input.as_ptr(),
                input.len(),
                ptr::null(),
                0,
            )
        };
        assert!(unsafe { error_text(result) }.contains("was not found"));

        let operation = CString::new("To Lower case").unwrap();
        let argument = CString::new("unexpected").unwrap();
        let argument = unsafe { rxchef_arg_str(argument.as_ptr()) };
        let arguments = [argument];
        let result = unsafe {
            rxchef_run(
                operation.as_ptr(),
                input.as_ptr(),
                input.len(),
                arguments.as_ptr(),
                arguments.len(),
            )
        };
        assert!(unsafe { error_text(result) }.contains("accepts 0 value"));
        unsafe { rxchef_free_arg(argument) };

        let operation = CString::new("From Base64").unwrap();
        let invalid = b"A";
        let result = unsafe {
            rxchef_run(
                operation.as_ptr(),
                invalid.as_ptr(),
                invalid.len(),
                ptr::null(),
                0,
            )
        };
        assert!(unsafe { error_text(result) }.contains("invalid input"));
    }

    #[test]
    fn allocations_are_independent_and_panic_boundary_is_tested() {
        let first = rxchef_list_operations();
        let second = rxchef_list_operations();
        assert!(!first.is_null());
        assert!(!second.is_null());
        assert_ne!(first, second);
        unsafe {
            rxchef_free_string(first);
            rxchef_free_string(second);
        }
        assert!(ffi_boundary(|| panic!("forced FFI boundary test")).is_err());
    }

    #[test]
    fn numeric_argument_rejects_non_finite_values() {
        assert!(rxchef_arg_num(f64::NAN).is_null());
        assert!(rxchef_arg_num(f64::INFINITY).is_null());
    }
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
