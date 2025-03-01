// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Thrown when you attempt to create more resources than the service allows based on service quotas.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ServiceQuotaExceededException {
    /// Description of the error.
    #[doc(hidden)]
    pub message: ::std::option::Option<::std::string::String>,
    /// Identifier of the resource affected.
    #[doc(hidden)]
    pub resource_id: ::std::option::Option<::std::string::String>,
    /// Type of the resource affected.
    #[doc(hidden)]
    pub resource_type: ::std::option::Option<::std::string::String>,
    /// Service Quotas requirement to identify originating service.
    #[doc(hidden)]
    pub service_code: ::std::option::Option<::std::string::String>,
    /// Service Quotas requirement to identify originating quota.
    #[doc(hidden)]
    pub quota_code: ::std::option::Option<::std::string::String>,
    pub(crate) meta: ::aws_smithy_types::error::ErrorMetadata,
}
impl ServiceQuotaExceededException {
    /// Identifier of the resource affected.
    pub fn resource_id(&self) -> ::std::option::Option<&str> {
        self.resource_id.as_deref()
    }
    /// Type of the resource affected.
    pub fn resource_type(&self) -> ::std::option::Option<&str> {
        self.resource_type.as_deref()
    }
    /// Service Quotas requirement to identify originating service.
    pub fn service_code(&self) -> ::std::option::Option<&str> {
        self.service_code.as_deref()
    }
    /// Service Quotas requirement to identify originating quota.
    pub fn quota_code(&self) -> ::std::option::Option<&str> {
        self.quota_code.as_deref()
    }
}
impl ServiceQuotaExceededException {
    /// Returns the error message.
    pub fn message(&self) -> ::std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl ::std::fmt::Display for ServiceQuotaExceededException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ::std::write!(f, "ServiceQuotaExceededException")?;
        if let ::std::option::Option::Some(inner_1) = &self.message {
            {
                ::std::write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl ::std::error::Error for ServiceQuotaExceededException {}
impl ::aws_http::request_id::RequestId for crate::types::error::ServiceQuotaExceededException {
    fn request_id(&self) -> Option<&str> {
        use ::aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for ServiceQuotaExceededException {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        &self.meta
    }
}
impl ServiceQuotaExceededException {
    /// Creates a new builder-style object to manufacture [`ServiceQuotaExceededException`](crate::types::error::ServiceQuotaExceededException).
    pub fn builder() -> crate::types::error::builders::ServiceQuotaExceededExceptionBuilder {
        crate::types::error::builders::ServiceQuotaExceededExceptionBuilder::default()
    }
}

/// A builder for [`ServiceQuotaExceededException`](crate::types::error::ServiceQuotaExceededException).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ServiceQuotaExceededExceptionBuilder {
    pub(crate) message: ::std::option::Option<::std::string::String>,
    pub(crate) resource_id: ::std::option::Option<::std::string::String>,
    pub(crate) resource_type: ::std::option::Option<::std::string::String>,
    pub(crate) service_code: ::std::option::Option<::std::string::String>,
    pub(crate) quota_code: ::std::option::Option<::std::string::String>,
    meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
}
impl ServiceQuotaExceededExceptionBuilder {
    /// Description of the error.
    pub fn message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message = ::std::option::Option::Some(input.into());
        self
    }
    /// Description of the error.
    pub fn set_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message = input;
        self
    }
    /// Identifier of the resource affected.
    pub fn resource_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource_id = ::std::option::Option::Some(input.into());
        self
    }
    /// Identifier of the resource affected.
    pub fn set_resource_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource_id = input;
        self
    }
    /// Type of the resource affected.
    pub fn resource_type(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.resource_type = ::std::option::Option::Some(input.into());
        self
    }
    /// Type of the resource affected.
    pub fn set_resource_type(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.resource_type = input;
        self
    }
    /// Service Quotas requirement to identify originating service.
    pub fn service_code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.service_code = ::std::option::Option::Some(input.into());
        self
    }
    /// Service Quotas requirement to identify originating service.
    pub fn set_service_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.service_code = input;
        self
    }
    /// Service Quotas requirement to identify originating quota.
    pub fn quota_code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.quota_code = ::std::option::Option::Some(input.into());
        self
    }
    /// Service Quotas requirement to identify originating quota.
    pub fn set_quota_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.quota_code = input;
        self
    }
    /// Sets error metadata
    pub fn meta(mut self, meta: ::aws_smithy_types::error::ErrorMetadata) -> Self {
        self.meta = Some(meta);
        self
    }

    /// Sets error metadata
    pub fn set_meta(
        &mut self,
        meta: std::option::Option<::aws_smithy_types::error::ErrorMetadata>,
    ) -> &mut Self {
        self.meta = meta;
        self
    }
    /// Consumes the builder and constructs a [`ServiceQuotaExceededException`](crate::types::error::ServiceQuotaExceededException).
    pub fn build(self) -> crate::types::error::ServiceQuotaExceededException {
        crate::types::error::ServiceQuotaExceededException {
            message: self.message,
            resource_id: self.resource_id,
            resource_type: self.resource_type,
            service_code: self.service_code,
            quota_code: self.quota_code,
            meta: self.meta.unwrap_or_default(),
        }
    }
}
