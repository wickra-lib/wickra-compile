//! The wickra-compile C ABI — the hub every C-capable language links against.
//!
//! The surface is tiny and JSON-shaped, exactly like [`compile_core::Compiler`]:
//! construct a handle, drive it with command JSONs (`compile`, `targets`,
//! `version`, `artifact_bytes`, `reset`), read back response JSONs, and free the
//! handle. No compile type crosses the boundary by value — the handle is opaque
//! and the payloads are always UTF-8 JSON strings.
//!
//! Responses use a caller-owned buffer with a length-out protocol (the classic
//! C two-call idiom): call with `out = NULL`, `cap = 0` to learn the length
//! `len`, then allocate `len + 1` and call again. When `len < cap` the response
//! is written immediately. Negative returns are reserved for unusable arguments
//! ([`WICKRA_COMPILE_ERR_NULL`], [`WICKRA_COMPILE_ERR_UTF8`]) and caught panics
//! ([`WICKRA_COMPILE_ERR_PANIC`]); a non-negative return is always the response
//! length. Domain errors come back in-band as `{"ok":false,"error":...}` JSON.
//!
//! Built artifact bytes are not returned in JSON: a `{"cmd":"artifact_bytes",...}`
//! response carries a `handle`, and [`wickra_compile_artifact_read`] copies the
//! raw bytes of that handle into a caller buffer.

use core::ffi::{c_char, CStr};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr;

use compile_core::Compiler;

/// A required pointer argument (`handle` or `cmd_json`) was null.
pub const WICKRA_COMPILE_ERR_NULL: i32 = -1;
/// `cmd_json` was not valid UTF-8.
pub const WICKRA_COMPILE_ERR_UTF8: i32 = -2;
/// A panic was caught at the FFI boundary.
pub const WICKRA_COMPILE_ERR_PANIC: i32 = -3;

/// An opaque handle to a compiler instance. Created by [`wickra_compile_new`]
/// and destroyed by [`wickra_compile_free`]; never dereferenced by the caller.
///
/// The handle caches the most recent command's response in `pending` so the
/// two-call length protocol does not execute the command twice — important
/// because some commands (`compile` with a real build, `artifact_bytes`) are
/// stateful. The cache is keyed on the raw command bytes and cleared once the
/// response has been delivered.
pub struct WickraCompiler {
    inner: Compiler,
    pending: Option<(Vec<u8>, String)>,
}

/// Read a NUL-terminated C string as `&str`, or `None` on null / bad UTF-8.
///
/// # Safety
/// `ptr` must be null or a valid NUL-terminated C string.
unsafe fn opt_str<'a>(ptr: *const c_char) -> Option<&'a str> {
    if ptr.is_null() {
        return None;
    }
    unsafe { CStr::from_ptr(ptr) }.to_str().ok()
}

/// Construct a compiler handle. Never null except on allocation failure. Free it
/// with [`wickra_compile_free`].
#[no_mangle]
pub extern "C" fn wickra_compile_new() -> *mut WickraCompiler {
    match catch_unwind(Compiler::new) {
        Ok(inner) => Box::into_raw(Box::new(WickraCompiler {
            inner,
            pending: None,
        })),
        Err(_) => ptr::null_mut(),
    }
}

/// Destroy a compiler handle. Null is a no-op.
///
/// # Safety
/// `handle` must be null or a handle previously returned by
/// [`wickra_compile_new`] and not already freed.
#[no_mangle]
pub unsafe extern "C" fn wickra_compile_free(handle: *mut WickraCompiler) {
    if !handle.is_null() {
        drop(unsafe { Box::from_raw(handle) });
    }
}

/// Apply a command JSON and write the response JSON into the caller's buffer.
///
/// Returns the response length in bytes (excluding the terminating NUL), or a
/// negative error code. When `len < cap`, the response and a trailing NUL have
/// been written to `out`; otherwise `out` is left untouched. Pass `out = NULL`,
/// `cap = 0` to query the length.
///
/// # Safety
/// `handle` must be a valid handle; `cmd_json` a valid NUL-terminated C string;
/// `out` either null or a writable buffer of at least `cap` bytes.
#[no_mangle]
pub unsafe extern "C" fn wickra_compile_command(
    handle: *mut WickraCompiler,
    cmd_json: *const c_char,
    out: *mut c_char,
    cap: usize,
) -> i32 {
    if handle.is_null() || cmd_json.is_null() {
        return WICKRA_COMPILE_ERR_NULL;
    }
    let Some(cmd) = (unsafe { opt_str(cmd_json) }) else {
        return WICKRA_COMPILE_ERR_UTF8;
    };
    let store = unsafe { &mut *handle };

    let is_retry = matches!(&store.pending, Some((bytes, _)) if bytes.as_slice() == cmd.as_bytes());
    if !is_retry {
        // `command_json` returns the response string directly (domain errors are
        // already in-band `{"ok":false,...}`); only a panic is exceptional.
        let Ok(response) = catch_unwind(AssertUnwindSafe(|| store.inner.command_json(cmd))) else {
            return WICKRA_COMPILE_ERR_PANIC;
        };
        store.pending = Some((cmd.as_bytes().to_vec(), response));
    }

    let (len, delivered) = {
        let response = &store.pending.as_ref().expect("pending set above").1;
        let bytes = response.as_bytes();
        let len = bytes.len();
        let delivered = len < cap && !out.is_null();
        if delivered {
            unsafe {
                ptr::copy_nonoverlapping(bytes.as_ptr(), out.cast::<u8>(), len);
                *out.add(len) = 0;
            }
        }
        (len, delivered)
    };
    if delivered {
        store.pending = None;
    }
    i32::try_from(len).unwrap_or(i32::MAX)
}

/// Copy the bytes of an open artifact handle into `out`.
///
/// The `byte_handle` is the `handle` field from a prior `artifact_bytes`
/// command's response. Returns the total byte length, or
/// [`WICKRA_COMPILE_ERR_NULL`] if the handle is unknown. When `len <= cap` the
/// bytes are copied; otherwise `out` is left untouched (the caller knows `len`
/// from the `artifact_bytes` response and re-calls).
///
/// # Safety
/// `handle` must be a valid handle; `out` either null or a writable buffer of at
/// least `cap` bytes.
#[no_mangle]
pub unsafe extern "C" fn wickra_compile_artifact_read(
    handle: *mut WickraCompiler,
    byte_handle: u64,
    out: *mut u8,
    cap: usize,
) -> i64 {
    if handle.is_null() {
        return i64::from(WICKRA_COMPILE_ERR_NULL);
    }
    let store = unsafe { &*handle };
    match store.inner.artifact_handle_bytes(byte_handle) {
        Some(bytes) => {
            let len = bytes.len();
            if len <= cap && !out.is_null() {
                unsafe { ptr::copy_nonoverlapping(bytes.as_ptr(), out, len) };
            }
            i64::try_from(len).unwrap_or(i64::MAX)
        }
        None => i64::from(WICKRA_COMPILE_ERR_NULL),
    }
}

/// The library version as a static NUL-terminated string (do not free).
#[no_mangle]
pub extern "C" fn wickra_compile_version() -> *const c_char {
    concat!(env!("CARGO_PKG_VERSION"), "\0")
        .as_ptr()
        .cast::<c_char>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    fn read_buf(buf: &[u8]) -> String {
        CStr::from_bytes_until_nul(buf)
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
    }

    const COMPILE_CMD: &str = r#"{"cmd":"compile","dry_run":true,"spec":{
        "strategy":{"symbol":"x","timeframe":"1h",
            "indicators":{"f":{"type":"Ema","params":[3]}},
            "entry":{"cross_above":["f","f"]},"exit":{"cross_below":["f","f"]},
            "sizing":{"type":"fixed_qty","qty":1}},
        "target":{"kind":"wasm"},"crate_name":"demo"}}"#;

    #[test]
    fn new_command_free_round_trip() {
        let handle = wickra_compile_new();
        assert!(!handle.is_null());
        let cmd = CString::new(COMPILE_CMD).unwrap();
        let len = unsafe { wickra_compile_command(handle, cmd.as_ptr(), ptr::null_mut(), 0) };
        assert!(len > 0);
        let mut buf = vec![0u8; usize::try_from(len).unwrap() + 1];
        let len2 = unsafe {
            wickra_compile_command(
                handle,
                cmd.as_ptr(),
                buf.as_mut_ptr().cast::<c_char>(),
                buf.len(),
            )
        };
        assert_eq!(len2, len);
        let response = read_buf(&buf);
        assert!(response.contains("\"project_hash\""));
        assert!(response.contains("\"built\":false"));
        unsafe { wickra_compile_free(handle) };
    }

    #[test]
    fn too_small_buffer_leaves_out_untouched() {
        let handle = wickra_compile_new();
        let cmd = CString::new(r#"{"cmd":"version"}"#).unwrap();
        let mut buf = vec![0xAAu8; 4];
        let len = unsafe {
            wickra_compile_command(
                handle,
                cmd.as_ptr(),
                buf.as_mut_ptr().cast::<c_char>(),
                buf.len(),
            )
        };
        assert!(usize::try_from(len).unwrap() >= buf.len());
        assert!(buf.iter().all(|&b| b == 0xAA));
        unsafe { wickra_compile_free(handle) };
    }

    #[test]
    fn unknown_cmd_is_in_band_error() {
        let handle = wickra_compile_new();
        let bad = CString::new(r#"{"cmd":"nope"}"#).unwrap();
        let len = unsafe { wickra_compile_command(handle, bad.as_ptr(), ptr::null_mut(), 0) };
        assert!(len > 0);
        let mut buf = vec![0u8; usize::try_from(len).unwrap() + 1];
        unsafe {
            wickra_compile_command(
                handle,
                bad.as_ptr(),
                buf.as_mut_ptr().cast::<c_char>(),
                buf.len(),
            );
        }
        assert!(read_buf(&buf).contains("\"ok\":false"));
        unsafe { wickra_compile_free(handle) };
    }

    #[test]
    fn null_guards() {
        let cmd = CString::new(r#"{"cmd":"version"}"#).unwrap();
        let code =
            unsafe { wickra_compile_command(ptr::null_mut(), cmd.as_ptr(), ptr::null_mut(), 0) };
        assert_eq!(code, WICKRA_COMPILE_ERR_NULL);
        let handle = wickra_compile_new();
        let code = unsafe { wickra_compile_command(handle, ptr::null(), ptr::null_mut(), 0) };
        assert_eq!(code, WICKRA_COMPILE_ERR_NULL);
        unsafe { wickra_compile_free(handle) };
    }

    #[test]
    fn unknown_artifact_handle_is_error() {
        let handle = wickra_compile_new();
        let code = unsafe { wickra_compile_artifact_read(handle, 999, ptr::null_mut(), 0) };
        assert_eq!(code, i64::from(WICKRA_COMPILE_ERR_NULL));
        unsafe { wickra_compile_free(handle) };
    }

    #[test]
    fn free_null_is_a_noop() {
        unsafe { wickra_compile_free(ptr::null_mut()) };
    }

    #[test]
    fn version_is_nul_terminated() {
        let p = wickra_compile_version();
        let v = unsafe { CStr::from_ptr(p) }.to_str().unwrap();
        assert_eq!(v, env!("CARGO_PKG_VERSION"));
    }
}
