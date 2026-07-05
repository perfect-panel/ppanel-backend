// Response envelopes and HTTP result construction.
//
// Ported from the Go package `result` together with its `xerr` dependency:
// error codes live in [`error_code`], the code-carrying error in
// [`code_error`], and the response beans / `HttpResult` in [`http_result`].

pub mod code_error;
pub mod error_code;
pub mod http_result;
