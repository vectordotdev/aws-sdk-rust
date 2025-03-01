// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListQualificationTypesOutput {
    /// <p> The number of Qualification types on this page in the filtered results list, equivalent to the number of types this operation returns. </p>
    #[doc(hidden)]
    pub num_results: ::std::option::Option<i32>,
    /// <p>If the previous response was incomplete (because there is more data to retrieve), Amazon Mechanical Turk returns a pagination token in the response. You can use this pagination token to retrieve the next set of results. </p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p> The list of QualificationType elements returned by the query. </p>
    #[doc(hidden)]
    pub qualification_types:
        ::std::option::Option<::std::vec::Vec<crate::types::QualificationType>>,
    _request_id: Option<String>,
}
impl ListQualificationTypesOutput {
    /// <p> The number of Qualification types on this page in the filtered results list, equivalent to the number of types this operation returns. </p>
    pub fn num_results(&self) -> ::std::option::Option<i32> {
        self.num_results
    }
    /// <p>If the previous response was incomplete (because there is more data to retrieve), Amazon Mechanical Turk returns a pagination token in the response. You can use this pagination token to retrieve the next set of results. </p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p> The list of QualificationType elements returned by the query. </p>
    pub fn qualification_types(&self) -> ::std::option::Option<&[crate::types::QualificationType]> {
        self.qualification_types.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for ListQualificationTypesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListQualificationTypesOutput {
    /// Creates a new builder-style object to manufacture [`ListQualificationTypesOutput`](crate::operation::list_qualification_types::ListQualificationTypesOutput).
    pub fn builder(
    ) -> crate::operation::list_qualification_types::builders::ListQualificationTypesOutputBuilder
    {
        crate::operation::list_qualification_types::builders::ListQualificationTypesOutputBuilder::default()
    }
}

/// A builder for [`ListQualificationTypesOutput`](crate::operation::list_qualification_types::ListQualificationTypesOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListQualificationTypesOutputBuilder {
    pub(crate) num_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) qualification_types:
        ::std::option::Option<::std::vec::Vec<crate::types::QualificationType>>,
    _request_id: Option<String>,
}
impl ListQualificationTypesOutputBuilder {
    /// <p> The number of Qualification types on this page in the filtered results list, equivalent to the number of types this operation returns. </p>
    pub fn num_results(mut self, input: i32) -> Self {
        self.num_results = ::std::option::Option::Some(input);
        self
    }
    /// <p> The number of Qualification types on this page in the filtered results list, equivalent to the number of types this operation returns. </p>
    pub fn set_num_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.num_results = input;
        self
    }
    /// <p>If the previous response was incomplete (because there is more data to retrieve), Amazon Mechanical Turk returns a pagination token in the response. You can use this pagination token to retrieve the next set of results. </p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If the previous response was incomplete (because there is more data to retrieve), Amazon Mechanical Turk returns a pagination token in the response. You can use this pagination token to retrieve the next set of results. </p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Appends an item to `qualification_types`.
    ///
    /// To override the contents of this collection use [`set_qualification_types`](Self::set_qualification_types).
    ///
    /// <p> The list of QualificationType elements returned by the query. </p>
    pub fn qualification_types(mut self, input: crate::types::QualificationType) -> Self {
        let mut v = self.qualification_types.unwrap_or_default();
        v.push(input);
        self.qualification_types = ::std::option::Option::Some(v);
        self
    }
    /// <p> The list of QualificationType elements returned by the query. </p>
    pub fn set_qualification_types(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::QualificationType>>,
    ) -> Self {
        self.qualification_types = input;
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
    /// Consumes the builder and constructs a [`ListQualificationTypesOutput`](crate::operation::list_qualification_types::ListQualificationTypesOutput).
    pub fn build(self) -> crate::operation::list_qualification_types::ListQualificationTypesOutput {
        crate::operation::list_qualification_types::ListQualificationTypesOutput {
            num_results: self.num_results,
            next_token: self.next_token,
            qualification_types: self.qualification_types,
            _request_id: self._request_id,
        }
    }
}
