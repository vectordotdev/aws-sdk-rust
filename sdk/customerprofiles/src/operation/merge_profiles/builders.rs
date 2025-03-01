// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::merge_profiles::_merge_profiles_output::MergeProfilesOutputBuilder;

pub use crate::operation::merge_profiles::_merge_profiles_input::MergeProfilesInputBuilder;

/// Fluent builder constructing a request to `MergeProfiles`.
///
/// <p>Runs an AWS Lambda job that does the following:</p>
/// <ol>
/// <li> <p>All the profileKeys in the <code>ProfileToBeMerged</code> will be moved to the main profile.</p> </li>
/// <li> <p>All the objects in the <code>ProfileToBeMerged</code> will be moved to the main profile.</p> </li>
/// <li> <p>All the <code>ProfileToBeMerged</code> will be deleted at the end.</p> </li>
/// <li> <p>All the profileKeys in the <code>ProfileIdsToBeMerged</code> will be moved to the main profile.</p> </li>
/// <li> <p>Standard fields are merged as follows:</p>
/// <ol>
/// <li> <p>Fields are always "union"-ed if there are no conflicts in standard fields or attributeKeys.</p> </li>
/// <li> <p>When there are conflicting fields:</p>
/// <ol>
/// <li> <p>If no <code>SourceProfileIds</code> entry is specified, the main Profile value is always taken. </p> </li>
/// <li> <p>If a <code>SourceProfileIds</code> entry is specified, the specified profileId is always taken, even if it is a NULL value.</p> </li>
/// </ol> </li>
/// </ol> </li>
/// </ol>
/// <p>You can use MergeProfiles together with <a href="https://docs.aws.amazon.com/customerprofiles/latest/APIReference/API_GetMatches.html">GetMatches</a>, which returns potentially matching profiles, or use it with the results of another matching system. After profiles have been merged, they cannot be separated (unmerged).</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct MergeProfilesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::merge_profiles::builders::MergeProfilesInputBuilder,
}
impl MergeProfilesFluentBuilder {
    /// Creates a new `MergeProfiles`.
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
            crate::operation::merge_profiles::MergeProfiles,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::merge_profiles::MergeProfilesError>,
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
        crate::operation::merge_profiles::MergeProfilesOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::merge_profiles::MergeProfilesError>,
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
        crate::operation::merge_profiles::MergeProfilesOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::merge_profiles::MergeProfilesError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::merge_profiles::MergeProfiles,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::merge_profiles::MergeProfilesError>,
    > {
        self.customize_middleware().await
    }
    /// <p>The unique name of the domain.</p>
    pub fn domain_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_name(input.into());
        self
    }
    /// <p>The unique name of the domain.</p>
    pub fn set_domain_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_name(input);
        self
    }
    /// <p>The identifier of the profile to be taken.</p>
    pub fn main_profile_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.main_profile_id(input.into());
        self
    }
    /// <p>The identifier of the profile to be taken.</p>
    pub fn set_main_profile_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_main_profile_id(input);
        self
    }
    /// Appends an item to `ProfileIdsToBeMerged`.
    ///
    /// To override the contents of this collection use [`set_profile_ids_to_be_merged`](Self::set_profile_ids_to_be_merged).
    ///
    /// <p>The identifier of the profile to be merged into MainProfileId.</p>
    pub fn profile_ids_to_be_merged(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.profile_ids_to_be_merged(input.into());
        self
    }
    /// <p>The identifier of the profile to be merged into MainProfileId.</p>
    pub fn set_profile_ids_to_be_merged(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_profile_ids_to_be_merged(input);
        self
    }
    /// <p>The identifiers of the fields in the profile that has the information you want to apply to the merge. For example, say you want to merge EmailAddress from Profile1 into MainProfile. This would be the identifier of the EmailAddress field in Profile1. </p>
    pub fn field_source_profile_ids(mut self, input: crate::types::FieldSourceProfileIds) -> Self {
        self.inner = self.inner.field_source_profile_ids(input);
        self
    }
    /// <p>The identifiers of the fields in the profile that has the information you want to apply to the merge. For example, say you want to merge EmailAddress from Profile1 into MainProfile. This would be the identifier of the EmailAddress field in Profile1. </p>
    pub fn set_field_source_profile_ids(
        mut self,
        input: ::std::option::Option<crate::types::FieldSourceProfileIds>,
    ) -> Self {
        self.inner = self.inner.set_field_source_profile_ids(input);
        self
    }
}
