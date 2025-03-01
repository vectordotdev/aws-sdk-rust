// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object that represents the specification of a virtual router.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct VirtualRouterSpec {
    /// <p>The listeners that the virtual router is expected to receive inbound traffic from. You can specify one listener.</p>
    #[doc(hidden)]
    pub listeners: ::std::option::Option<::std::vec::Vec<crate::types::VirtualRouterListener>>,
}
impl VirtualRouterSpec {
    /// <p>The listeners that the virtual router is expected to receive inbound traffic from. You can specify one listener.</p>
    pub fn listeners(&self) -> ::std::option::Option<&[crate::types::VirtualRouterListener]> {
        self.listeners.as_deref()
    }
}
impl VirtualRouterSpec {
    /// Creates a new builder-style object to manufacture [`VirtualRouterSpec`](crate::types::VirtualRouterSpec).
    pub fn builder() -> crate::types::builders::VirtualRouterSpecBuilder {
        crate::types::builders::VirtualRouterSpecBuilder::default()
    }
}

/// A builder for [`VirtualRouterSpec`](crate::types::VirtualRouterSpec).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct VirtualRouterSpecBuilder {
    pub(crate) listeners:
        ::std::option::Option<::std::vec::Vec<crate::types::VirtualRouterListener>>,
}
impl VirtualRouterSpecBuilder {
    /// Appends an item to `listeners`.
    ///
    /// To override the contents of this collection use [`set_listeners`](Self::set_listeners).
    ///
    /// <p>The listeners that the virtual router is expected to receive inbound traffic from. You can specify one listener.</p>
    pub fn listeners(mut self, input: crate::types::VirtualRouterListener) -> Self {
        let mut v = self.listeners.unwrap_or_default();
        v.push(input);
        self.listeners = ::std::option::Option::Some(v);
        self
    }
    /// <p>The listeners that the virtual router is expected to receive inbound traffic from. You can specify one listener.</p>
    pub fn set_listeners(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::VirtualRouterListener>>,
    ) -> Self {
        self.listeners = input;
        self
    }
    /// Consumes the builder and constructs a [`VirtualRouterSpec`](crate::types::VirtualRouterSpec).
    pub fn build(self) -> crate::types::VirtualRouterSpec {
        crate::types::VirtualRouterSpec {
            listeners: self.listeners,
        }
    }
}
