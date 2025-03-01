// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeVirtualNodeOutput {
    /// <p>The full description of your virtual node.</p>
    #[doc(hidden)]
    pub virtual_node: ::std::option::Option<crate::types::VirtualNodeData>,
    _request_id: Option<String>,
}
impl DescribeVirtualNodeOutput {
    /// <p>The full description of your virtual node.</p>
    pub fn virtual_node(&self) -> ::std::option::Option<&crate::types::VirtualNodeData> {
        self.virtual_node.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeVirtualNodeOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeVirtualNodeOutput {
    /// Creates a new builder-style object to manufacture [`DescribeVirtualNodeOutput`](crate::operation::describe_virtual_node::DescribeVirtualNodeOutput).
    pub fn builder(
    ) -> crate::operation::describe_virtual_node::builders::DescribeVirtualNodeOutputBuilder {
        crate::operation::describe_virtual_node::builders::DescribeVirtualNodeOutputBuilder::default(
        )
    }
}

/// A builder for [`DescribeVirtualNodeOutput`](crate::operation::describe_virtual_node::DescribeVirtualNodeOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeVirtualNodeOutputBuilder {
    pub(crate) virtual_node: ::std::option::Option<crate::types::VirtualNodeData>,
    _request_id: Option<String>,
}
impl DescribeVirtualNodeOutputBuilder {
    /// <p>The full description of your virtual node.</p>
    pub fn virtual_node(mut self, input: crate::types::VirtualNodeData) -> Self {
        self.virtual_node = ::std::option::Option::Some(input);
        self
    }
    /// <p>The full description of your virtual node.</p>
    pub fn set_virtual_node(
        mut self,
        input: ::std::option::Option<crate::types::VirtualNodeData>,
    ) -> Self {
        self.virtual_node = input;
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
    /// Consumes the builder and constructs a [`DescribeVirtualNodeOutput`](crate::operation::describe_virtual_node::DescribeVirtualNodeOutput).
    pub fn build(self) -> crate::operation::describe_virtual_node::DescribeVirtualNodeOutput {
        crate::operation::describe_virtual_node::DescribeVirtualNodeOutput {
            virtual_node: self.virtual_node,
            _request_id: self._request_id,
        }
    }
}
