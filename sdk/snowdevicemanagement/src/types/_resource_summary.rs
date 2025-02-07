// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A summary of a resource available on the device.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ResourceSummary {
    /// <p>The resource type.</p>
    #[doc(hidden)]
    pub resource_type: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the resource.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
}
impl ResourceSummary {
    /// <p>The resource type.</p>
    pub fn resource_type(&self) -> ::std::option::Option<&str> {
        self.resource_type.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>The ID of the resource.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
}
impl ResourceSummary {
    /// Creates a new builder-style object to manufacture [`ResourceSummary`](crate::types::ResourceSummary).
    pub fn builder() -> crate::types::builders::ResourceSummaryBuilder {
        crate::types::builders::ResourceSummaryBuilder::default()
    }
}

/// A builder for [`ResourceSummary`](crate::types::ResourceSummary).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ResourceSummaryBuilder {
    pub(crate) resource_type: ::std::option::Option<::std::string::String>,
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) id: ::std::option::Option<::std::string::String>,
}
impl ResourceSummaryBuilder {
    /// <p>The resource type.</p>
    pub fn resource_type(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.resource_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The resource type.</p>
    pub fn set_resource_type(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.resource_type = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The ID of the resource.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the resource.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// Consumes the builder and constructs a [`ResourceSummary`](crate::types::ResourceSummary).
    pub fn build(self) -> crate::types::ResourceSummary {
        crate::types::ResourceSummary {
            resource_type: self.resource_type,
            arn: self.arn,
            id: self.id,
        }
    }
}
