// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateEnvironmentOutput {
    /// <p>The unique identifier for FinSpace environment that you created.</p>
    #[doc(hidden)]
    pub environment_id: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the FinSpace environment that you created.</p>
    #[doc(hidden)]
    pub environment_arn: ::std::option::Option<::std::string::String>,
    /// <p>The sign-in url for the web application of the FinSpace environment you created.</p>
    #[doc(hidden)]
    pub environment_url: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CreateEnvironmentOutput {
    /// <p>The unique identifier for FinSpace environment that you created.</p>
    pub fn environment_id(&self) -> ::std::option::Option<&str> {
        self.environment_id.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the FinSpace environment that you created.</p>
    pub fn environment_arn(&self) -> ::std::option::Option<&str> {
        self.environment_arn.as_deref()
    }
    /// <p>The sign-in url for the web application of the FinSpace environment you created.</p>
    pub fn environment_url(&self) -> ::std::option::Option<&str> {
        self.environment_url.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for CreateEnvironmentOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateEnvironmentOutput {
    /// Creates a new builder-style object to manufacture [`CreateEnvironmentOutput`](crate::operation::create_environment::CreateEnvironmentOutput).
    pub fn builder(
    ) -> crate::operation::create_environment::builders::CreateEnvironmentOutputBuilder {
        crate::operation::create_environment::builders::CreateEnvironmentOutputBuilder::default()
    }
}

/// A builder for [`CreateEnvironmentOutput`](crate::operation::create_environment::CreateEnvironmentOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateEnvironmentOutputBuilder {
    pub(crate) environment_id: ::std::option::Option<::std::string::String>,
    pub(crate) environment_arn: ::std::option::Option<::std::string::String>,
    pub(crate) environment_url: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CreateEnvironmentOutputBuilder {
    /// <p>The unique identifier for FinSpace environment that you created.</p>
    pub fn environment_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.environment_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier for FinSpace environment that you created.</p>
    pub fn set_environment_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.environment_id = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the FinSpace environment that you created.</p>
    pub fn environment_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.environment_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the FinSpace environment that you created.</p>
    pub fn set_environment_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.environment_arn = input;
        self
    }
    /// <p>The sign-in url for the web application of the FinSpace environment you created.</p>
    pub fn environment_url(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.environment_url = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The sign-in url for the web application of the FinSpace environment you created.</p>
    pub fn set_environment_url(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.environment_url = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`CreateEnvironmentOutput`](crate::operation::create_environment::CreateEnvironmentOutput).
    pub fn build(self) -> crate::operation::create_environment::CreateEnvironmentOutput {
        crate::operation::create_environment::CreateEnvironmentOutput {
            environment_id: self.environment_id,
            environment_arn: self.environment_arn,
            environment_url: self.environment_url,
            _request_id: self._request_id,
        }
    }
}
