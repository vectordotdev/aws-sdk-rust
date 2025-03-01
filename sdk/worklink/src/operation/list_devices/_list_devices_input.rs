// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListDevicesInput {
    /// <p>The ARN of the fleet.</p>
    #[doc(hidden)]
    pub fleet_arn: ::std::option::Option<::std::string::String>,
    /// <p>The pagination token used to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of results to be included in the next page.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
}
impl ListDevicesInput {
    /// <p>The ARN of the fleet.</p>
    pub fn fleet_arn(&self) -> ::std::option::Option<&str> {
        self.fleet_arn.as_deref()
    }
    /// <p>The pagination token used to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of results to be included in the next page.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
}
impl ListDevicesInput {
    /// Creates a new builder-style object to manufacture [`ListDevicesInput`](crate::operation::list_devices::ListDevicesInput).
    pub fn builder() -> crate::operation::list_devices::builders::ListDevicesInputBuilder {
        crate::operation::list_devices::builders::ListDevicesInputBuilder::default()
    }
}

/// A builder for [`ListDevicesInput`](crate::operation::list_devices::ListDevicesInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListDevicesInputBuilder {
    pub(crate) fleet_arn: ::std::option::Option<::std::string::String>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
}
impl ListDevicesInputBuilder {
    /// <p>The ARN of the fleet.</p>
    pub fn fleet_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.fleet_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the fleet.</p>
    pub fn set_fleet_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.fleet_arn = input;
        self
    }
    /// <p>The pagination token used to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The pagination token used to retrieve the next page of results for this operation. If this value is null, it retrieves the first page.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The maximum number of results to be included in the next page.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of results to be included in the next page.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// Consumes the builder and constructs a [`ListDevicesInput`](crate::operation::list_devices::ListDevicesInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_devices::ListDevicesInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_devices::ListDevicesInput {
            fleet_arn: self.fleet_arn,
            next_token: self.next_token,
            max_results: self.max_results,
        })
    }
}
