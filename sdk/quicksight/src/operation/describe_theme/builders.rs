// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_theme::_describe_theme_output::DescribeThemeOutputBuilder;

pub use crate::operation::describe_theme::_describe_theme_input::DescribeThemeInputBuilder;

/// Fluent builder constructing a request to `DescribeTheme`.
///
/// <p>Describes a theme.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeThemeFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_theme::builders::DescribeThemeInputBuilder,
}
impl DescribeThemeFluentBuilder {
    /// Creates a new `DescribeTheme`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn customize_middleware(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::describe_theme::DescribeTheme,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::describe_theme::DescribeThemeError>,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        ::std::result::Result::Ok(crate::client::customize::CustomizableOperation {
            handle,
            operation,
        })
    }

    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn send_middleware(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_theme::DescribeThemeOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::describe_theme::DescribeThemeError>,
    > {
        let op = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_theme::DescribeThemeOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::describe_theme::DescribeThemeError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::describe_theme::DescribeTheme,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::describe_theme::DescribeThemeError>,
    > {
        self.customize_middleware().await
    }
    /// <p>The ID of the Amazon Web Services account that contains the theme that you're describing.</p>
    pub fn aws_account_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.aws_account_id(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account that contains the theme that you're describing.</p>
    pub fn set_aws_account_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_aws_account_id(input);
        self
    }
    /// <p>The ID for the theme.</p>
    pub fn theme_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.theme_id(input.into());
        self
    }
    /// <p>The ID for the theme.</p>
    pub fn set_theme_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_theme_id(input);
        self
    }
    /// <p>The version number for the version to describe. If a <code>VersionNumber</code> parameter value isn't provided, the latest version of the theme is described.</p>
    pub fn version_number(mut self, input: i64) -> Self {
        self.inner = self.inner.version_number(input);
        self
    }
    /// <p>The version number for the version to describe. If a <code>VersionNumber</code> parameter value isn't provided, the latest version of the theme is described.</p>
    pub fn set_version_number(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_version_number(input);
        self
    }
    /// <p>The alias of the theme that you want to describe. If you name a specific alias, you describe the version that the alias points to. You can specify the latest version of the theme by providing the keyword <code>$LATEST</code> in the <code>AliasName</code> parameter. The keyword <code>$PUBLISHED</code> doesn't apply to themes.</p>
    pub fn alias_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.alias_name(input.into());
        self
    }
    /// <p>The alias of the theme that you want to describe. If you name a specific alias, you describe the version that the alias points to. You can specify the latest version of the theme by providing the keyword <code>$LATEST</code> in the <code>AliasName</code> parameter. The keyword <code>$PUBLISHED</code> doesn't apply to themes.</p>
    pub fn set_alias_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_alias_name(input);
        self
    }
}
