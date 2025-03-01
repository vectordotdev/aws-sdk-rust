// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains details about the protected query output.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub enum ProtectedQueryOutput {
    /// <p>If present, the output for a protected query with an `S3` output type.</p>
    S3(crate::types::ProtectedQueryS3Output),
    /// The `Unknown` variant represents cases where new union variant was received. Consider upgrading the SDK to the latest available version.
    /// An unknown enum variant
    ///
    /// _Note: If you encounter this error, consider upgrading your SDK to the latest version._
    /// The `Unknown` variant represents cases where the server sent a value that wasn't recognized
    /// by the client. This can happen when the server adds new functionality, but the client has not been updated.
    /// To investigate this, consider turning on debug logging to print the raw HTTP response.
    #[non_exhaustive]
    Unknown,
}
impl ProtectedQueryOutput {
    #[allow(irrefutable_let_patterns)]
    /// Tries to convert the enum instance into [`S3`](crate::types::ProtectedQueryOutput::S3), extracting the inner [`ProtectedQueryS3Output`](crate::types::ProtectedQueryS3Output).
    /// Returns `Err(&Self)` if it can't be converted.
    pub fn as_s3(&self) -> ::std::result::Result<&crate::types::ProtectedQueryS3Output, &Self> {
        if let ProtectedQueryOutput::S3(val) = &self {
            ::std::result::Result::Ok(val)
        } else {
            ::std::result::Result::Err(self)
        }
    }
    /// Returns true if this is a [`S3`](crate::types::ProtectedQueryOutput::S3).
    pub fn is_s3(&self) -> bool {
        self.as_s3().is_ok()
    }
    /// Returns true if the enum instance is the `Unknown` variant.
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}
