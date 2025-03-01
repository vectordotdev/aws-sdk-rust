// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteGameOutput {
    _request_id: Option<String>,
}
impl ::aws_http::request_id::RequestId for DeleteGameOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteGameOutput {
    /// Creates a new builder-style object to manufacture [`DeleteGameOutput`](crate::operation::delete_game::DeleteGameOutput).
    pub fn builder() -> crate::operation::delete_game::builders::DeleteGameOutputBuilder {
        crate::operation::delete_game::builders::DeleteGameOutputBuilder::default()
    }
}

/// A builder for [`DeleteGameOutput`](crate::operation::delete_game::DeleteGameOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteGameOutputBuilder {
    _request_id: Option<String>,
}
impl DeleteGameOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeleteGameOutput`](crate::operation::delete_game::DeleteGameOutput).
    pub fn build(self) -> crate::operation::delete_game::DeleteGameOutput {
        crate::operation::delete_game::DeleteGameOutput {
            _request_id: self._request_id,
        }
    }
}
