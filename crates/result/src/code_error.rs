// Code-carrying error type.
//
// Ported from the Go package `xerr` (errors.go). A `CodeError` carries a
// numeric error code (shown to the front end) and a human-readable message,
// and can travel through an error chain so handlers can recover the code via
// [`find_code_error`].

use std::fmt;
use std::sync::LazyLock;

use crate::error_code::{map_err_msg, ERROR};

/// An error that carries a machine-readable code and a message.
#[derive(Debug, Clone)]
pub struct CodeError {
    err_code: u32,
    err_msg: String,
}

impl CodeError {
    /// Creates a `CodeError` whose message is looked up from [`map_err_msg`].
    ///
    /// Mirrors Go `xerr.NewErrCode`.
    pub fn new_err_code(err_code: u32) -> Self {
        Self {
            err_code,
            err_msg: map_err_msg(err_code).to_string(),
        }
    }

    /// Creates a `CodeError` with an explicit code and message.
    ///
    /// Mirrors Go `xerr.NewErrCodeMsg`.
    pub fn new_err_code_msg(err_code: u32, err_msg: impl Into<String>) -> Self {
        Self {
            err_code,
            err_msg: err_msg.into(),
        }
    }

    /// Creates a `CodeError` for an unspecified failure (`ERROR` code).
    ///
    /// Mirrors Go `xerr.NewErrMsg`.
    pub fn new_err_msg(err_msg: impl Into<String>) -> Self {
        Self {
            err_code: ERROR,
            err_msg: err_msg.into(),
        }
    }

    /// Returns the error code shown to the front end.
    pub fn get_err_code(&self) -> u32 {
        self.err_code
    }

    /// Returns the error message shown to the front end.
    pub fn get_err_msg(&self) -> &str {
        &self.err_msg
    }
}

impl fmt::Display for CodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Preserve the original Go format (note the full-width comma).
        write!(f, "ErrCode:{}，ErrMsg:{}", self.err_code, self.err_msg)
    }
}

impl std::error::Error for CodeError {}

/// Sentinel error for "304 Not Modified".
///
/// Mirrors Go `xerr.StatusNotModified`.
pub static STATUS_NOT_MODIFIED: LazyLock<SimpleError> =
    LazyLock::new(|| SimpleError("304 Not Modified".to_string()));

/// A plain string-backed error, used for sentinel values such as
/// [`STATUS_NOT_MODIFIED`].
#[derive(Debug, Clone)]
pub struct SimpleError(pub String);

impl fmt::Display for SimpleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for SimpleError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_err_code_maps_message() {
        let e = CodeError::new_err_code(crate::error_code::INVALID_PARAMS);
        assert_eq!(e.get_err_code(), 400);
        assert_eq!(e.get_err_msg(), "Param Error");
    }

    #[test]
    fn new_err_code_msg_is_explicit() {
        let e = CodeError::new_err_code_msg(123, "custom");
        assert_eq!(e.get_err_code(), 123);
        assert_eq!(e.get_err_msg(), "custom");
    }

    #[test]
    fn new_err_msg_uses_error_code() {
        let e = CodeError::new_err_msg("boom");
        assert_eq!(e.get_err_code(), ERROR);
        assert_eq!(e.get_err_msg(), "boom");
    }

    #[test]
    fn display_preserves_go_format() {
        let e = CodeError::new_err_code_msg(400, "Param Error");
        assert_eq!(e.to_string(), "ErrCode:400，ErrMsg:Param Error");
    }
}
