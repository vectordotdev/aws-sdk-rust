// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_microsoft_ad::_create_microsoft_ad_output::CreateMicrosoftAdOutputBuilder;

pub use crate::operation::create_microsoft_ad::_create_microsoft_ad_input::CreateMicrosoftAdInputBuilder;

/// Fluent builder constructing a request to `CreateMicrosoftAD`.
///
/// <p>Creates a Microsoft AD directory in the Amazon Web Services Cloud. For more information, see <a href="https://docs.aws.amazon.com/directoryservice/latest/admin-guide/directory_microsoft_ad.html">Managed Microsoft AD</a> in the <i>Directory Service Admin Guide</i>.</p>
/// <p>Before you call <i>CreateMicrosoftAD</i>, ensure that all of the required permissions have been explicitly granted through a policy. For details about what permissions are required to run the <i>CreateMicrosoftAD</i> operation, see <a href="http://docs.aws.amazon.com/directoryservice/latest/admin-guide/UsingWithDS_IAM_ResourcePermissions.html">Directory Service API Permissions: Actions, Resources, and Conditions Reference</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateMicrosoftADFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_microsoft_ad::builders::CreateMicrosoftAdInputBuilder,
}
impl CreateMicrosoftADFluentBuilder {
    /// Creates a new `CreateMicrosoftAD`.
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
            crate::operation::create_microsoft_ad::CreateMicrosoftAD,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_microsoft_ad::CreateMicrosoftADError,
        >,
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
        crate::operation::create_microsoft_ad::CreateMicrosoftAdOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_microsoft_ad::CreateMicrosoftADError,
        >,
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
        crate::operation::create_microsoft_ad::CreateMicrosoftAdOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_microsoft_ad::CreateMicrosoftADError,
        >,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::create_microsoft_ad::CreateMicrosoftAD,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_microsoft_ad::CreateMicrosoftADError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The fully qualified domain name for the Managed Microsoft AD directory, such as <code>corp.example.com</code>. This name will resolve inside your VPC only. It does not need to be publicly resolvable.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The fully qualified domain name for the Managed Microsoft AD directory, such as <code>corp.example.com</code>. This name will resolve inside your VPC only. It does not need to be publicly resolvable.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The NetBIOS name for your domain, such as <code>CORP</code>. If you don't specify a NetBIOS name, it will default to the first part of your directory DNS. For example, <code>CORP</code> for the directory DNS <code>corp.example.com</code>. </p>
    pub fn short_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.short_name(input.into());
        self
    }
    /// <p>The NetBIOS name for your domain, such as <code>CORP</code>. If you don't specify a NetBIOS name, it will default to the first part of your directory DNS. For example, <code>CORP</code> for the directory DNS <code>corp.example.com</code>. </p>
    pub fn set_short_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_short_name(input);
        self
    }
    /// <p>The password for the default administrative user named <code>Admin</code>.</p>
    /// <p>If you need to change the password for the administrator account, you can use the <code>ResetUserPassword</code> API call.</p>
    pub fn password(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.password(input.into());
        self
    }
    /// <p>The password for the default administrative user named <code>Admin</code>.</p>
    /// <p>If you need to change the password for the administrator account, you can use the <code>ResetUserPassword</code> API call.</p>
    pub fn set_password(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_password(input);
        self
    }
    /// <p>A description for the directory. This label will appear on the Amazon Web Services console <code>Directory Details</code> page after the directory is created.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description for the directory. This label will appear on the Amazon Web Services console <code>Directory Details</code> page after the directory is created.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>Contains VPC information for the <code>CreateDirectory</code> or <code>CreateMicrosoftAD</code> operation.</p>
    pub fn vpc_settings(mut self, input: crate::types::DirectoryVpcSettings) -> Self {
        self.inner = self.inner.vpc_settings(input);
        self
    }
    /// <p>Contains VPC information for the <code>CreateDirectory</code> or <code>CreateMicrosoftAD</code> operation.</p>
    pub fn set_vpc_settings(
        mut self,
        input: ::std::option::Option<crate::types::DirectoryVpcSettings>,
    ) -> Self {
        self.inner = self.inner.set_vpc_settings(input);
        self
    }
    /// <p>Managed Microsoft AD is available in two editions: <code>Standard</code> and <code>Enterprise</code>. <code>Enterprise</code> is the default.</p>
    pub fn edition(mut self, input: crate::types::DirectoryEdition) -> Self {
        self.inner = self.inner.edition(input);
        self
    }
    /// <p>Managed Microsoft AD is available in two editions: <code>Standard</code> and <code>Enterprise</code>. <code>Enterprise</code> is the default.</p>
    pub fn set_edition(
        mut self,
        input: ::std::option::Option<crate::types::DirectoryEdition>,
    ) -> Self {
        self.inner = self.inner.set_edition(input);
        self
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags to be assigned to the Managed Microsoft AD directory.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>The tags to be assigned to the Managed Microsoft AD directory.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
