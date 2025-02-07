// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetSegmentExportJobsInput {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    #[doc(hidden)]
    pub application_id: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of items to include in each page of a paginated response. This parameter is not supported for application, campaign, and journey metrics.</p>
    #[doc(hidden)]
    pub page_size: ::std::option::Option<::std::string::String>,
    /// <p>The unique identifier for the segment.</p>
    #[doc(hidden)]
    pub segment_id: ::std::option::Option<::std::string::String>,
    /// <p>The NextToken string that specifies which page of results to return in a paginated response.</p>
    #[doc(hidden)]
    pub token: ::std::option::Option<::std::string::String>,
}
impl GetSegmentExportJobsInput {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    pub fn application_id(&self) -> ::std::option::Option<&str> {
        self.application_id.as_deref()
    }
    /// <p>The maximum number of items to include in each page of a paginated response. This parameter is not supported for application, campaign, and journey metrics.</p>
    pub fn page_size(&self) -> ::std::option::Option<&str> {
        self.page_size.as_deref()
    }
    /// <p>The unique identifier for the segment.</p>
    pub fn segment_id(&self) -> ::std::option::Option<&str> {
        self.segment_id.as_deref()
    }
    /// <p>The NextToken string that specifies which page of results to return in a paginated response.</p>
    pub fn token(&self) -> ::std::option::Option<&str> {
        self.token.as_deref()
    }
}
impl GetSegmentExportJobsInput {
    /// Creates a new builder-style object to manufacture [`GetSegmentExportJobsInput`](crate::operation::get_segment_export_jobs::GetSegmentExportJobsInput).
    pub fn builder(
    ) -> crate::operation::get_segment_export_jobs::builders::GetSegmentExportJobsInputBuilder {
        crate::operation::get_segment_export_jobs::builders::GetSegmentExportJobsInputBuilder::default()
    }
}

/// A builder for [`GetSegmentExportJobsInput`](crate::operation::get_segment_export_jobs::GetSegmentExportJobsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetSegmentExportJobsInputBuilder {
    pub(crate) application_id: ::std::option::Option<::std::string::String>,
    pub(crate) page_size: ::std::option::Option<::std::string::String>,
    pub(crate) segment_id: ::std::option::Option<::std::string::String>,
    pub(crate) token: ::std::option::Option<::std::string::String>,
}
impl GetSegmentExportJobsInputBuilder {
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    pub fn application_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.application_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    pub fn set_application_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.application_id = input;
        self
    }
    /// <p>The maximum number of items to include in each page of a paginated response. This parameter is not supported for application, campaign, and journey metrics.</p>
    pub fn page_size(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.page_size = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The maximum number of items to include in each page of a paginated response. This parameter is not supported for application, campaign, and journey metrics.</p>
    pub fn set_page_size(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.page_size = input;
        self
    }
    /// <p>The unique identifier for the segment.</p>
    pub fn segment_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.segment_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier for the segment.</p>
    pub fn set_segment_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.segment_id = input;
        self
    }
    /// <p>The NextToken string that specifies which page of results to return in a paginated response.</p>
    pub fn token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The NextToken string that specifies which page of results to return in a paginated response.</p>
    pub fn set_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.token = input;
        self
    }
    /// Consumes the builder and constructs a [`GetSegmentExportJobsInput`](crate::operation::get_segment_export_jobs::GetSegmentExportJobsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_segment_export_jobs::GetSegmentExportJobsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_segment_export_jobs::GetSegmentExportJobsInput {
                application_id: self.application_id,
                page_size: self.page_size,
                segment_id: self.segment_id,
                token: self.token,
            },
        )
    }
}
