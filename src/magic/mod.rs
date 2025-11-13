#![allow(unsafe_code)]
// from :: https://github.com/robo9k/rust-magic/blob/main/src/ffi.rs

pub mod magic_sys;
pub use magic_sys::*;

pub const MAGIC_MIME: u32 = MAGIC_MIME_TYPE | MAGIC_MIME_ENCODING;
pub const MAGIC_NODESC: u32 = MAGIC_EXTENSION | MAGIC_MIME | MAGIC_APPLE;
pub const MAGIC_NO_CHECK_BUILTIN: u32 = MAGIC_NO_CHECK_COMPRESS |
    MAGIC_NO_CHECK_TAR      |
    // MAGIC_NO_CHECK_SOFT  |
    MAGIC_NO_CHECK_APPTYPE  |
    MAGIC_NO_CHECK_ELF      |
    MAGIC_NO_CHECK_TEXT     |
    MAGIC_NO_CHECK_CSV      |
    MAGIC_NO_CHECK_CDF      |
    MAGIC_NO_CHECK_TOKENS   |
    MAGIC_NO_CHECK_ENCODING |
    MAGIC_NO_CHECK_JSON;

// NOTE: the following are from `file.h`, but part of `magic.h` API
pub const FILE_LOAD: u32 = 0;
pub const FILE_CHECK: u32 = 1;
pub const FILE_COMPILE: u32 = 2;
pub const FILE_LIST: u32 = 3;

#[derive(Debug)]
// non-copy wrapper around raw pointer
#[repr(transparent)]
pub(crate) struct Cookie(magic_sys::magic_t);

impl Cookie {
    pub fn new(cookie: &mut Self) -> Self {
        Self(cookie.0)
    }
}

/// Error for opened `magic_t` instance
#[derive(thiserror::Error, Debug)]
#[error("magic cookie error ({}): {}",
match .errno {
    Some(errno) => format!("OS errno: {}", errno),
    None => "no OS errno".to_string(),
},
.explanation.to_string_lossy()
)]
pub(crate) struct CookieError {
    pub explanation: std::ffi::CString,
    errno: Option<std::io::Error>,
}

fn last_error(cookie: &Cookie) -> Option<CookieError> {
    let error = unsafe { magic_sys::magic_error(cookie.0) };
    let errno = unsafe { magic_sys::magic_errno(cookie.0) };

    if error.is_null() {
        None
    } else {
        let c_str = unsafe { std::ffi::CStr::from_ptr(error) };
        Some(CookieError {
            explanation: c_str.into(),
            errno: match errno {
                0 => None,
                _ => Some(std::io::Error::from_raw_os_error(errno)),
            },
        })
    }
}

fn api_violation(cookie: &Cookie, description: String) -> ! {
    panic!(
        "`magic_sys` API violation for magic cookie {:?}: {}",
        cookie, description
    );
}

fn expect_error(cookie: &Cookie, description: String) -> CookieError {
    match last_error(cookie) {
        Some(err) => err,
        _ => api_violation(cookie, description),
    }
}

pub(crate) fn close(cookie: &mut Cookie) {
    unsafe { magic_sys::magic_close(cookie.0) }
}

/// # Panics
///
/// Panics if `magic_sys` violates its API contract, e.g. by not setting the last error.
pub(crate) fn file(
    cookie: &Cookie,
    filename: &std::ffi::CStr, // TODO: Support NULL
) -> Result<std::ffi::CString, CookieError> {
    let filename_ptr = filename.as_ptr();
    let res = unsafe { magic_sys::magic_file(cookie.0, filename_ptr) };

    if res.is_null() {
        Err(expect_error(
            cookie,
            "`magic_file()` did not set last error".to_string(),
        ))
    } else {
        let c_str = unsafe { std::ffi::CStr::from_ptr(res) };
        Ok(c_str.into())
    }
}

/// # Panics
///
/// Panics if `magic_sys` violates its API contract, e.g. by not setting the last error.
pub(crate) fn buffer(cookie: &Cookie, buffer: &[u8]) -> Result<std::ffi::CString, CookieError> {
    let buffer_ptr = buffer.as_ptr();
    let buffer_len = buffer.len() as libc::size_t;
    let res = unsafe { magic_sys::magic_buffer(cookie.0, buffer_ptr as _, buffer_len) };

    if res.is_null() {
        Err(expect_error(
            cookie,
            "`magic_buffer()` did not set last error".to_string(),
        ))
    } else {
        let c_str = unsafe { std::ffi::CStr::from_ptr(res) };
        Ok(c_str.into())
    }
}

pub(crate) fn setflags(cookie: &Cookie, flags: libc::c_int) -> Result<(), SetFlagsError> {
    let ret = unsafe { magic_sys::magic_setflags(cookie.0, flags) };
    match ret {
        -1 => Err(SetFlagsError { flags }),
        _ => Ok(()),
    }
}

#[derive(thiserror::Error, Debug)]
#[error("could not set magic cookie flags {}", .flags)]
pub(crate) struct SetFlagsError {
    flags: libc::c_int,
}

/// # Panics
///
/// Panics if `magic_sys` violates its API contract, e.g. by not setting the last error or returning undefined data.
pub(crate) fn check(cookie: &Cookie, filename: Option<&std::ffi::CStr>) -> Result<(), CookieError> {
    let filename_ptr = filename.map_or_else(std::ptr::null, std::ffi::CStr::as_ptr);
    let res = unsafe { magic_sys::magic_check(cookie.0, filename_ptr) };

    match res {
        0 => Ok(()),
        -1 => Err(expect_error(
            cookie,
            "`magic_check()` did not set last error".to_string(),
        )),
        res => api_violation(
            cookie,
            format!("expected 0 or -1 but `magic_check()` returned {}", res),
        ),
    }
}

/// # Panics
///
/// Panics if `magic_sys` violates its API contract, e.g. by not setting the last error or returning undefined data.
pub(crate) fn compile(
    cookie: &Cookie,
    filename: Option<&std::ffi::CStr>,
) -> Result<(), CookieError> {
    let filename_ptr = filename.map_or_else(std::ptr::null, std::ffi::CStr::as_ptr);
    let res = unsafe { magic_sys::magic_compile(cookie.0, filename_ptr) };

    match res {
        0 => Ok(()),
        -1 => Err(expect_error(
            cookie,
            "`magic_compile()` did not set last error".to_string(),
        )),
        res => api_violation(
            cookie,
            format!("Expected 0 or -1 but `magic_compile()` returned {}", res),
        ),
    }
}

/// # Panics
///
/// Panics if `magic_sys` violates its API contract, e.g. by not setting the last error or returning undefined data.
pub(crate) fn list(cookie: &Cookie, filename: Option<&std::ffi::CStr>) -> Result<(), CookieError> {
    let filename_ptr = filename.map_or_else(std::ptr::null, std::ffi::CStr::as_ptr);
    let res = unsafe { magic_sys::magic_list(cookie.0, filename_ptr) };

    match res {
        0 => Ok(()),
        -1 => Err(expect_error(
            cookie,
            "`magic_list()` did not set last error".to_string(),
        )),
        res => api_violation(
            cookie,
            format!("Expected 0 or -1 but `magic_list()` returned {}", res),
        ),
    }
}

/// # Panics
///
/// Panics if `magic_sys` violates its API contract, e.g. by not setting the last error or returning undefined data.
pub(crate) fn load(cookie: &Cookie, filename: Option<&std::ffi::CStr>) -> Result<(), CookieError> {
    let filename_ptr = filename.map_or_else(std::ptr::null, std::ffi::CStr::as_ptr);
    let res = unsafe { magic_sys::magic_load(cookie.0, filename_ptr) };

    match res {
        0 => Ok(()),
        -1 => Err(expect_error(
            cookie,
            "`magic_load()` did not set last error".to_string(),
        )),
        res => api_violation(
            cookie,
            format!("Expected 0 or -1 but `magic_load()` returned {}", res),
        ),
    }
}

/// # Panics
///
/// Panics if `magic_sys` violates its API contract, e.g. by not setting the last error or returning undefined data.
pub(crate) fn load_buffers(cookie: &Cookie, buffers: &[&[u8]]) -> Result<(), CookieError> {
    let mut ffi_buffers: Vec<*const u8> = Vec::with_capacity(buffers.len());
    let mut ffi_sizes: Vec<libc::size_t> = Vec::with_capacity(buffers.len());
    let ffi_nbuffers = buffers.len() as libc::size_t;

    for slice in buffers {
        ffi_buffers.push((*slice).as_ptr());
        ffi_sizes.push(slice.len() as libc::size_t);
    }

    let ffi_buffers_ptr = ffi_buffers.as_mut_ptr() as *mut *mut libc::c_void;
    let ffi_sizes_ptr = ffi_sizes.as_mut_ptr();

    let res = unsafe {
        magic_sys::magic_load_buffers(cookie.0, ffi_buffers_ptr, ffi_sizes_ptr, ffi_nbuffers)
    };

    match res {
        0 => Ok(()),
        -1 => Err(expect_error(
            cookie,
            "`magic_load_buffers()` did not set last error".to_string(),
        )),
        res => api_violation(
            cookie,
            format!(
                "Expected 0 or -1 but `magic_load_buffers()` returned {}",
                res
            ),
        ),
    }
}

pub(crate) fn open(flags: libc::c_int) -> Result<Cookie, OpenError> {
    let cookie = unsafe { magic_sys::magic_open(flags) };

    if cookie.is_null() {
        Err(OpenError {
            flags,
            // note that magic_sys only really cares about MAGIC_PRESERVE_ATIME
            // invalid bits in `flags` still result in a valid cookie pointer
            errno: std::io::Error::last_os_error(),
        })
    } else {
        Ok(Cookie(cookie))
    }
}

#[derive(thiserror::Error, Debug)]
#[error("could not open magic cookie with flags {}: {}", .flags, .errno)]
pub(crate) struct OpenError {
    flags: libc::c_int,
    errno: std::io::Error,
}

impl OpenError {
    pub fn errno(&self) -> &std::io::Error {
        &self.errno
    }
}

pub(crate) fn version() -> libc::c_int {
    unsafe { magic_sys::magic_version() }
}
