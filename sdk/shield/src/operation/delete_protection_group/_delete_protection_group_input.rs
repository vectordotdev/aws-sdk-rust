// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteProtectionGroupInput {
    /// <p>The name of the protection group. You use this to identify the protection group in lists and to manage the protection group, for example to update, delete, or describe it. </p>
    #[doc(hidden)]
    pub protection_group_id: ::std::option::Option<::std::string::String>,
}
impl DeleteProtectionGroupInput {
    /// <p>The name of the protection group. You use this to identify the protection group in lists and to manage the protection group, for example to update, delete, or describe it. </p>
    pub fn protection_group_id(&self) -> ::std::option::Option<&str> {
        self.protection_group_id.as_deref()
    }
}
impl DeleteProtectionGroupInput {
    /// Creates a new builder-style object to manufacture [`DeleteProtectionGroupInput`](crate::operation::delete_protection_group::DeleteProtectionGroupInput).
    pub fn builder(
    ) -> crate::operation::delete_protection_group::builders::DeleteProtectionGroupInputBuilder
    {
        crate::operation::delete_protection_group::builders::DeleteProtectionGroupInputBuilder::default()
    }
}

/// A builder for [`DeleteProtectionGroupInput`](crate::operation::delete_protection_group::DeleteProtectionGroupInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteProtectionGroupInputBuilder {
    pub(crate) protection_group_id: ::std::option::Option<::std::string::String>,
}
impl DeleteProtectionGroupInputBuilder {
    /// <p>The name of the protection group. You use this to identify the protection group in lists and to manage the protection group, for example to update, delete, or describe it. </p>
    pub fn protection_group_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.protection_group_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the protection group. You use this to identify the protection group in lists and to manage the protection group, for example to update, delete, or describe it. </p>
    pub fn set_protection_group_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.protection_group_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteProtectionGroupInput`](crate::operation::delete_protection_group::DeleteProtectionGroupInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_protection_group::DeleteProtectionGroupInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::delete_protection_group::DeleteProtectionGroupInput {
                protection_group_id: self.protection_group_id,
            },
        )
    }
}
