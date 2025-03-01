// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeUserStackAssociationsOutput {
    /// <p>The UserStackAssociation objects.</p>
    #[doc(hidden)]
    pub user_stack_associations:
        ::std::option::Option<::std::vec::Vec<crate::types::UserStackAssociation>>,
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeUserStackAssociationsOutput {
    /// <p>The UserStackAssociation objects.</p>
    pub fn user_stack_associations(
        &self,
    ) -> ::std::option::Option<&[crate::types::UserStackAssociation]> {
        self.user_stack_associations.as_deref()
    }
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeUserStackAssociationsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeUserStackAssociationsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeUserStackAssociationsOutput`](crate::operation::describe_user_stack_associations::DescribeUserStackAssociationsOutput).
    pub fn builder() -> crate::operation::describe_user_stack_associations::builders::DescribeUserStackAssociationsOutputBuilder{
        crate::operation::describe_user_stack_associations::builders::DescribeUserStackAssociationsOutputBuilder::default()
    }
}

/// A builder for [`DescribeUserStackAssociationsOutput`](crate::operation::describe_user_stack_associations::DescribeUserStackAssociationsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeUserStackAssociationsOutputBuilder {
    pub(crate) user_stack_associations:
        ::std::option::Option<::std::vec::Vec<crate::types::UserStackAssociation>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeUserStackAssociationsOutputBuilder {
    /// Appends an item to `user_stack_associations`.
    ///
    /// To override the contents of this collection use [`set_user_stack_associations`](Self::set_user_stack_associations).
    ///
    /// <p>The UserStackAssociation objects.</p>
    pub fn user_stack_associations(mut self, input: crate::types::UserStackAssociation) -> Self {
        let mut v = self.user_stack_associations.unwrap_or_default();
        v.push(input);
        self.user_stack_associations = ::std::option::Option::Some(v);
        self
    }
    /// <p>The UserStackAssociation objects.</p>
    pub fn set_user_stack_associations(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::UserStackAssociation>>,
    ) -> Self {
        self.user_stack_associations = input;
        self
    }
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The pagination token to use to retrieve the next page of results for this operation. If there are no more pages, this value is null.</p>
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
    /// Consumes the builder and constructs a [`DescribeUserStackAssociationsOutput`](crate::operation::describe_user_stack_associations::DescribeUserStackAssociationsOutput).
    pub fn build(
        self,
    ) -> crate::operation::describe_user_stack_associations::DescribeUserStackAssociationsOutput
    {
        crate::operation::describe_user_stack_associations::DescribeUserStackAssociationsOutput {
            user_stack_associations: self.user_stack_associations,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
