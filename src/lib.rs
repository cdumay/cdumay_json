//! [![License: BSD-3-Clause](https://img.shields.io/badge/license-BSD--3--Clause-blue)](./LICENSE)
//! [![cdumay_json on crates.io](https://img.shields.io/crates/v/cdumay_json)](https://crates.io/crates/cdumay_json)
//! [![cdumay_json on docs.rs](https://docs.rs/cdumay_json/badge.svg)](https://docs.rs/cdumay_json)
//! [![Source Code Repository](https://img.shields.io/badge/Code-On%20GitHub-blue?logo=GitHub)](https://github.com/cdumay/cdumay_json)
//!
//! A small crate to manipulate Json data.
//!
//! ## Features
//!
//! - Categorizes `serde_json::Error` into specific error types (`Syntax`, `IO`, `Data`, `EOF`)
//! - Each error type is associated with a custom code, HTTP status, and descriptive message
//! - Structured output for APIs, logging systems, and observability platforms
//! - Includes context metadata via `BTreeMap`
//! - Provides a convenient `convert_json_result!` macro for error conversion
//!
//! ## Usage
//!
//! Using the `JsonErrorConverter` directly:
//! ```rust
//! use cdumay_core::{Error, ErrorConverter};
//! use serde_json::Value;
//! use std::collections::BTreeMap;
//! use cdumay_json::JsonErrorConverter;
//!
//! fn parse_json(input: &str) -> cdumay_core::Result<Value> {
//!     serde_json::from_str::<Value>(input).map_err(|e| {
//!        let mut ctx = BTreeMap::new();
//!        ctx.insert("input".to_string(), serde_value::Value::String(input.to_string()));
//!        JsonErrorConverter::convert(&e, "Failed to parse JSON".to_string(), ctx)
//!    })
//! }
//! ```
//!
//! Using the `convert_json_result!` macro:
//! ```rust
//! use cdumay_json::convert_json_result;
//! use serde_json::Value;
//! use std::collections::BTreeMap;
//! use cdumay_core::{Error, ErrorConverter};
//!
//! fn parse_json(input: &str) -> cdumay_core::Result<Value> {
//!     // Basic usage with just the result
//!     convert_json_result!(serde_json::from_str::<Value>(input));
//!
//!     // With custom context
//!     let mut ctx = BTreeMap::new();
//!     ctx.insert("input".to_string(), serde_value::Value::String(input.to_string()));
//!     convert_json_result!(serde_json::from_str::<Value>(input), ctx.clone());
//!
//!     // With custom context and message
//!     convert_json_result!(serde_json::from_str::<Value>(input), ctx, "Failed to parse JSON")
//! }
//! ```
#[macro_use]
mod macros;

use cdumay_core::{Error, ErrorConverter, define_errors, define_kinds};
use serde_json::error::Category;
use std::collections::BTreeMap;

define_kinds! {
    JsonSyntax = (400, "Syntax Error"),
    JsonData = (400, "Invalid JSON data"),
    JsonEof = (500, "Reached the end of the input data"),
    JsonIo = (500, "IO Error"),
}

define_errors! {
    IoError = JsonIo,
    SyntaxError = JsonSyntax,
    DataError = JsonData,
    EofError = JsonEof
}

/// A utility struct for handling JSON errors and converting them into standardized error types.
pub struct JsonErrorConverter;

impl ErrorConverter for JsonErrorConverter {
    type Error = serde_json::Error;
    /// Converts a `serde_json::Error` into a standardized `Error` type based on its category.
    ///
    /// # Arguments
    ///
    /// * `err` - The `serde_json::Error` to be converted.
    /// * `text` - A descriptive message for the error.
    /// * `context` - A mutable reference to a `BTreeMap` containing additional error details.
    ///
    /// # Returns
    ///
    /// A standardized `Error` instance corresponding to the category of the provided `serde_json::Error`.
    fn convert(err: &serde_json::Error, text: String, context: BTreeMap<String, serde_value::Value>) -> Error {
        match err.classify() {
            Category::Io => IoError::new().with_message(text).with_details(context).into(),
            Category::Syntax => SyntaxError::new().with_message(text).with_details(context).into(),
            Category::Data => DataError::new().with_message(text).with_details(context).into(),
            Category::Eof => EofError::new().with_message(text).with_details(context).into(),
        }
    }
}
