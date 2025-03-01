// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListResourceProfileArtifactsOutput {
    /// <p>An array of objects, one for each of 1-100 S3 objects that Amazon Macie selected for analysis.</p>
    /// <p>If Macie has analyzed more than 100 objects in the bucket, Macie populates the array based on the value for the ResourceProfileArtifact.sensitive field for an object: true (sensitive), followed by false (not sensitive). Macie then populates any remaining items in the array with information about objects where the value for the ResourceProfileArtifact.classificationResultStatus field is SKIPPED.</p>
    #[doc(hidden)]
    pub artifacts: ::std::option::Option<::std::vec::Vec<crate::types::ResourceProfileArtifact>>,
    /// <p>The string to use in a subsequent request to get the next page of results in a paginated response. This value is null if there are no additional pages.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListResourceProfileArtifactsOutput {
    /// <p>An array of objects, one for each of 1-100 S3 objects that Amazon Macie selected for analysis.</p>
    /// <p>If Macie has analyzed more than 100 objects in the bucket, Macie populates the array based on the value for the ResourceProfileArtifact.sensitive field for an object: true (sensitive), followed by false (not sensitive). Macie then populates any remaining items in the array with information about objects where the value for the ResourceProfileArtifact.classificationResultStatus field is SKIPPED.</p>
    pub fn artifacts(&self) -> ::std::option::Option<&[crate::types::ResourceProfileArtifact]> {
        self.artifacts.as_deref()
    }
    /// <p>The string to use in a subsequent request to get the next page of results in a paginated response. This value is null if there are no additional pages.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListResourceProfileArtifactsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListResourceProfileArtifactsOutput {
    /// Creates a new builder-style object to manufacture [`ListResourceProfileArtifactsOutput`](crate::operation::list_resource_profile_artifacts::ListResourceProfileArtifactsOutput).
    pub fn builder() -> crate::operation::list_resource_profile_artifacts::builders::ListResourceProfileArtifactsOutputBuilder{
        crate::operation::list_resource_profile_artifacts::builders::ListResourceProfileArtifactsOutputBuilder::default()
    }
}

/// A builder for [`ListResourceProfileArtifactsOutput`](crate::operation::list_resource_profile_artifacts::ListResourceProfileArtifactsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListResourceProfileArtifactsOutputBuilder {
    pub(crate) artifacts:
        ::std::option::Option<::std::vec::Vec<crate::types::ResourceProfileArtifact>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListResourceProfileArtifactsOutputBuilder {
    /// Appends an item to `artifacts`.
    ///
    /// To override the contents of this collection use [`set_artifacts`](Self::set_artifacts).
    ///
    /// <p>An array of objects, one for each of 1-100 S3 objects that Amazon Macie selected for analysis.</p>
    /// <p>If Macie has analyzed more than 100 objects in the bucket, Macie populates the array based on the value for the ResourceProfileArtifact.sensitive field for an object: true (sensitive), followed by false (not sensitive). Macie then populates any remaining items in the array with information about objects where the value for the ResourceProfileArtifact.classificationResultStatus field is SKIPPED.</p>
    pub fn artifacts(mut self, input: crate::types::ResourceProfileArtifact) -> Self {
        let mut v = self.artifacts.unwrap_or_default();
        v.push(input);
        self.artifacts = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of objects, one for each of 1-100 S3 objects that Amazon Macie selected for analysis.</p>
    /// <p>If Macie has analyzed more than 100 objects in the bucket, Macie populates the array based on the value for the ResourceProfileArtifact.sensitive field for an object: true (sensitive), followed by false (not sensitive). Macie then populates any remaining items in the array with information about objects where the value for the ResourceProfileArtifact.classificationResultStatus field is SKIPPED.</p>
    pub fn set_artifacts(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ResourceProfileArtifact>>,
    ) -> Self {
        self.artifacts = input;
        self
    }
    /// <p>The string to use in a subsequent request to get the next page of results in a paginated response. This value is null if there are no additional pages.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The string to use in a subsequent request to get the next page of results in a paginated response. This value is null if there are no additional pages.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
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
    /// Consumes the builder and constructs a [`ListResourceProfileArtifactsOutput`](crate::operation::list_resource_profile_artifacts::ListResourceProfileArtifactsOutput).
    pub fn build(
        self,
    ) -> crate::operation::list_resource_profile_artifacts::ListResourceProfileArtifactsOutput {
        crate::operation::list_resource_profile_artifacts::ListResourceProfileArtifactsOutput {
            artifacts: self.artifacts,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
