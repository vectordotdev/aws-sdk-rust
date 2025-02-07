// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutAppInstanceStreamingConfigurationsInput {
    /// <p>The ARN of the <code>AppInstance</code>.</p>
    #[doc(hidden)]
    pub app_instance_arn: ::std::option::Option<::std::string::String>,
    /// <p>The streaming configurations set for an <code>AppInstance</code>.</p>
    #[doc(hidden)]
    pub app_instance_streaming_configurations:
        ::std::option::Option<::std::vec::Vec<crate::types::AppInstanceStreamingConfiguration>>,
}
impl PutAppInstanceStreamingConfigurationsInput {
    /// <p>The ARN of the <code>AppInstance</code>.</p>
    pub fn app_instance_arn(&self) -> ::std::option::Option<&str> {
        self.app_instance_arn.as_deref()
    }
    /// <p>The streaming configurations set for an <code>AppInstance</code>.</p>
    pub fn app_instance_streaming_configurations(
        &self,
    ) -> ::std::option::Option<&[crate::types::AppInstanceStreamingConfiguration]> {
        self.app_instance_streaming_configurations.as_deref()
    }
}
impl PutAppInstanceStreamingConfigurationsInput {
    /// Creates a new builder-style object to manufacture [`PutAppInstanceStreamingConfigurationsInput`](crate::operation::put_app_instance_streaming_configurations::PutAppInstanceStreamingConfigurationsInput).
    pub fn builder() -> crate::operation::put_app_instance_streaming_configurations::builders::PutAppInstanceStreamingConfigurationsInputBuilder{
        crate::operation::put_app_instance_streaming_configurations::builders::PutAppInstanceStreamingConfigurationsInputBuilder::default()
    }
}

/// A builder for [`PutAppInstanceStreamingConfigurationsInput`](crate::operation::put_app_instance_streaming_configurations::PutAppInstanceStreamingConfigurationsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PutAppInstanceStreamingConfigurationsInputBuilder {
    pub(crate) app_instance_arn: ::std::option::Option<::std::string::String>,
    pub(crate) app_instance_streaming_configurations:
        ::std::option::Option<::std::vec::Vec<crate::types::AppInstanceStreamingConfiguration>>,
}
impl PutAppInstanceStreamingConfigurationsInputBuilder {
    /// <p>The ARN of the <code>AppInstance</code>.</p>
    pub fn app_instance_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.app_instance_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the <code>AppInstance</code>.</p>
    pub fn set_app_instance_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.app_instance_arn = input;
        self
    }
    /// Appends an item to `app_instance_streaming_configurations`.
    ///
    /// To override the contents of this collection use [`set_app_instance_streaming_configurations`](Self::set_app_instance_streaming_configurations).
    ///
    /// <p>The streaming configurations set for an <code>AppInstance</code>.</p>
    pub fn app_instance_streaming_configurations(
        mut self,
        input: crate::types::AppInstanceStreamingConfiguration,
    ) -> Self {
        let mut v = self
            .app_instance_streaming_configurations
            .unwrap_or_default();
        v.push(input);
        self.app_instance_streaming_configurations = ::std::option::Option::Some(v);
        self
    }
    /// <p>The streaming configurations set for an <code>AppInstance</code>.</p>
    pub fn set_app_instance_streaming_configurations(
        mut self,
        input: ::std::option::Option<
            ::std::vec::Vec<crate::types::AppInstanceStreamingConfiguration>,
        >,
    ) -> Self {
        self.app_instance_streaming_configurations = input;
        self
    }
    /// Consumes the builder and constructs a [`PutAppInstanceStreamingConfigurationsInput`](crate::operation::put_app_instance_streaming_configurations::PutAppInstanceStreamingConfigurationsInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::put_app_instance_streaming_configurations::PutAppInstanceStreamingConfigurationsInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::put_app_instance_streaming_configurations::PutAppInstanceStreamingConfigurationsInput {
                app_instance_arn: self.app_instance_arn
                ,
                app_instance_streaming_configurations: self.app_instance_streaming_configurations
                ,
            }
        )
    }
}
