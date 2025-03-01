// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetAppInstanceRetentionSettingsOutput {
    /// <p>The retention settings for the <code>AppInstance</code>.</p>
    #[doc(hidden)]
    pub app_instance_retention_settings:
        ::std::option::Option<crate::types::AppInstanceRetentionSettings>,
    /// <p>The timestamp representing the time at which the specified items are retained, in Epoch Seconds.</p>
    #[doc(hidden)]
    pub initiate_deletion_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    _request_id: Option<String>,
}
impl GetAppInstanceRetentionSettingsOutput {
    /// <p>The retention settings for the <code>AppInstance</code>.</p>
    pub fn app_instance_retention_settings(
        &self,
    ) -> ::std::option::Option<&crate::types::AppInstanceRetentionSettings> {
        self.app_instance_retention_settings.as_ref()
    }
    /// <p>The timestamp representing the time at which the specified items are retained, in Epoch Seconds.</p>
    pub fn initiate_deletion_timestamp(
        &self,
    ) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.initiate_deletion_timestamp.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for GetAppInstanceRetentionSettingsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetAppInstanceRetentionSettingsOutput {
    /// Creates a new builder-style object to manufacture [`GetAppInstanceRetentionSettingsOutput`](crate::operation::get_app_instance_retention_settings::GetAppInstanceRetentionSettingsOutput).
    pub fn builder() -> crate::operation::get_app_instance_retention_settings::builders::GetAppInstanceRetentionSettingsOutputBuilder{
        crate::operation::get_app_instance_retention_settings::builders::GetAppInstanceRetentionSettingsOutputBuilder::default()
    }
}

/// A builder for [`GetAppInstanceRetentionSettingsOutput`](crate::operation::get_app_instance_retention_settings::GetAppInstanceRetentionSettingsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetAppInstanceRetentionSettingsOutputBuilder {
    pub(crate) app_instance_retention_settings:
        ::std::option::Option<crate::types::AppInstanceRetentionSettings>,
    pub(crate) initiate_deletion_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    _request_id: Option<String>,
}
impl GetAppInstanceRetentionSettingsOutputBuilder {
    /// <p>The retention settings for the <code>AppInstance</code>.</p>
    pub fn app_instance_retention_settings(
        mut self,
        input: crate::types::AppInstanceRetentionSettings,
    ) -> Self {
        self.app_instance_retention_settings = ::std::option::Option::Some(input);
        self
    }
    /// <p>The retention settings for the <code>AppInstance</code>.</p>
    pub fn set_app_instance_retention_settings(
        mut self,
        input: ::std::option::Option<crate::types::AppInstanceRetentionSettings>,
    ) -> Self {
        self.app_instance_retention_settings = input;
        self
    }
    /// <p>The timestamp representing the time at which the specified items are retained, in Epoch Seconds.</p>
    pub fn initiate_deletion_timestamp(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.initiate_deletion_timestamp = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timestamp representing the time at which the specified items are retained, in Epoch Seconds.</p>
    pub fn set_initiate_deletion_timestamp(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.initiate_deletion_timestamp = input;
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
    /// Consumes the builder and constructs a [`GetAppInstanceRetentionSettingsOutput`](crate::operation::get_app_instance_retention_settings::GetAppInstanceRetentionSettingsOutput).
    pub fn build(
        self,
    ) -> crate::operation::get_app_instance_retention_settings::GetAppInstanceRetentionSettingsOutput
    {
        crate::operation::get_app_instance_retention_settings::GetAppInstanceRetentionSettingsOutput {
            app_instance_retention_settings: self.app_instance_retention_settings
            ,
            initiate_deletion_timestamp: self.initiate_deletion_timestamp
            ,
            _request_id: self._request_id,
        }
    }
}
