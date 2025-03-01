// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteRotationOverrideInput {
    /// <p>The Amazon Resource Name (ARN) of the rotation that was overridden.</p>
    #[doc(hidden)]
    pub rotation_id: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the on-call rotation override to delete.</p>
    #[doc(hidden)]
    pub rotation_override_id: ::std::option::Option<::std::string::String>,
}
impl DeleteRotationOverrideInput {
    /// <p>The Amazon Resource Name (ARN) of the rotation that was overridden.</p>
    pub fn rotation_id(&self) -> ::std::option::Option<&str> {
        self.rotation_id.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the on-call rotation override to delete.</p>
    pub fn rotation_override_id(&self) -> ::std::option::Option<&str> {
        self.rotation_override_id.as_deref()
    }
}
impl DeleteRotationOverrideInput {
    /// Creates a new builder-style object to manufacture [`DeleteRotationOverrideInput`](crate::operation::delete_rotation_override::DeleteRotationOverrideInput).
    pub fn builder(
    ) -> crate::operation::delete_rotation_override::builders::DeleteRotationOverrideInputBuilder
    {
        crate::operation::delete_rotation_override::builders::DeleteRotationOverrideInputBuilder::default()
    }
}

/// A builder for [`DeleteRotationOverrideInput`](crate::operation::delete_rotation_override::DeleteRotationOverrideInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteRotationOverrideInputBuilder {
    pub(crate) rotation_id: ::std::option::Option<::std::string::String>,
    pub(crate) rotation_override_id: ::std::option::Option<::std::string::String>,
}
impl DeleteRotationOverrideInputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the rotation that was overridden.</p>
    pub fn rotation_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.rotation_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the rotation that was overridden.</p>
    pub fn set_rotation_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.rotation_id = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the on-call rotation override to delete.</p>
    pub fn rotation_override_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.rotation_override_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the on-call rotation override to delete.</p>
    pub fn set_rotation_override_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.rotation_override_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteRotationOverrideInput`](crate::operation::delete_rotation_override::DeleteRotationOverrideInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_rotation_override::DeleteRotationOverrideInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::delete_rotation_override::DeleteRotationOverrideInput {
                rotation_id: self.rotation_id,
                rotation_override_id: self.rotation_override_id,
            },
        )
    }
}
