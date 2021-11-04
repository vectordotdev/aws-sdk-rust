#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <p>Welcome to the Audit Manager API reference. This guide is for developers who
//! need detailed information about the Audit Manager API operations, data types, and
//! errors. </p>
//! <p>Audit Manager is a service that provides automated evidence collection so that
//! you can continually audit your Amazon Web Services usage. You can use it to assess the
//! effectiveness of your controls, manage risk, and simplify compliance.</p>
//! <p>Audit Manager provides prebuilt frameworks that structure and automate
//! assessments for a given compliance standard. Frameworks include a prebuilt collection of
//! controls with descriptions and testing procedures. These controls are grouped according to
//! the requirements of the specified compliance standard or regulation. You can also customize
//! frameworks and controls to support internal audits with specific requirements. </p>
//! <p>Use the following links to get started with the Audit Manager API:</p>
//! <ul>
//! <li>
//! <p>
//! <a href="https://docs.aws.amazon.com/audit-manager/latest/APIReference/API_Operations.html">Actions</a>: An
//! alphabetical list of all Audit Manager API operations.</p>
//! </li>
//! <li>
//! <p>
//! <a href="https://docs.aws.amazon.com/audit-manager/latest/APIReference/API_Types.html">Data types</a>: An alphabetical list of all Audit Manager data
//! types.</p>
//! </li>
//! <li>
//! <p>
//! <a href="https://docs.aws.amazon.com/audit-manager/latest/APIReference/CommonParameters.html">Common
//! parameters</a>: Parameters that all Query operations can use.</p>
//! </li>
//! <li>
//! <p>
//! <a href="https://docs.aws.amazon.com/audit-manager/latest/APIReference/CommonErrors.html">Common errors</a>:
//! Client and server errors that all operations can return.</p>
//! </li>
//! </ul>
//! <p>If you're new to Audit Manager, we recommend that you review the <a href="https://docs.aws.amazon.com/audit-manager/latest/userguide/what-is.html">
//! Audit Manager User Guide</a>.</p>

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

pub use config::Config;

mod aws_endpoint;
/// Client and fluent builders for calling the service.
#[cfg(feature = "client")]
pub mod client;
/// Configuration for the service.
pub mod config;
/// Errors that can occur when calling the service.
pub mod error;
mod error_meta;
/// Input structures for operations.
pub mod input;
mod json_deser;
mod json_errors;
mod json_ser;
/// Data structures used by operation inputs/outputs.
pub mod model;
mod no_credentials;
/// All operations that this crate can perform.
pub mod operation;
mod operation_deser;
mod operation_ser;
/// Output structures for operations.
pub mod output;
/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub use aws_smithy_http::byte_stream::ByteStream;
pub use aws_smithy_http::result::SdkError;
pub use aws_smithy_types::Blob;
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("auditmanager", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_smithy_types::retry::RetryConfig;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[cfg(feature = "client")]
pub use client::Client;
