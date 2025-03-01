// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetFaceLivenessSessionResultsInput {
    /// <p>A unique 128-bit UUID. This is used to uniquely identify the session and also acts as an idempotency token for all operations associated with the session.</p>
    #[doc(hidden)]
    pub session_id: ::std::option::Option<::std::string::String>,
}
impl GetFaceLivenessSessionResultsInput {
    /// <p>A unique 128-bit UUID. This is used to uniquely identify the session and also acts as an idempotency token for all operations associated with the session.</p>
    pub fn session_id(&self) -> ::std::option::Option<&str> {
        self.session_id.as_deref()
    }
}
impl GetFaceLivenessSessionResultsInput {
    /// Creates a new builder-style object to manufacture [`GetFaceLivenessSessionResultsInput`](crate::operation::get_face_liveness_session_results::GetFaceLivenessSessionResultsInput).
    pub fn builder() -> crate::operation::get_face_liveness_session_results::builders::GetFaceLivenessSessionResultsInputBuilder{
        crate::operation::get_face_liveness_session_results::builders::GetFaceLivenessSessionResultsInputBuilder::default()
    }
}

/// A builder for [`GetFaceLivenessSessionResultsInput`](crate::operation::get_face_liveness_session_results::GetFaceLivenessSessionResultsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetFaceLivenessSessionResultsInputBuilder {
    pub(crate) session_id: ::std::option::Option<::std::string::String>,
}
impl GetFaceLivenessSessionResultsInputBuilder {
    /// <p>A unique 128-bit UUID. This is used to uniquely identify the session and also acts as an idempotency token for all operations associated with the session.</p>
    pub fn session_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.session_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique 128-bit UUID. This is used to uniquely identify the session and also acts as an idempotency token for all operations associated with the session.</p>
    pub fn set_session_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.session_id = input;
        self
    }
    /// Consumes the builder and constructs a [`GetFaceLivenessSessionResultsInput`](crate::operation::get_face_liveness_session_results::GetFaceLivenessSessionResultsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_face_liveness_session_results::GetFaceLivenessSessionResultsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_face_liveness_session_results::GetFaceLivenessSessionResultsInput {
                session_id: self.session_id
                ,
            }
        )
    }
}
