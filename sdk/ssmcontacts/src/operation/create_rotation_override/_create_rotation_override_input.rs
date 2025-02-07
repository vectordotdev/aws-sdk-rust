// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateRotationOverrideInput {
    /// <p>The Amazon Resource Name (ARN) of the rotation to create an override for.</p>
    #[doc(hidden)]
    pub rotation_id: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Names (ARNs) of the contacts to replace those in the current on-call rotation with.</p>
    /// <p>If you want to include any current team members in the override shift, you must include their ARNs in the new contact ID list.</p>
    #[doc(hidden)]
    pub new_contact_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The date and time when the override goes into effect.</p>
    #[doc(hidden)]
    pub start_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The date and time when the override ends.</p>
    #[doc(hidden)]
    pub end_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>A token that ensures that the operation is called only once with the specified details.</p>
    #[doc(hidden)]
    pub idempotency_token: ::std::option::Option<::std::string::String>,
}
impl CreateRotationOverrideInput {
    /// <p>The Amazon Resource Name (ARN) of the rotation to create an override for.</p>
    pub fn rotation_id(&self) -> ::std::option::Option<&str> {
        self.rotation_id.as_deref()
    }
    /// <p>The Amazon Resource Names (ARNs) of the contacts to replace those in the current on-call rotation with.</p>
    /// <p>If you want to include any current team members in the override shift, you must include their ARNs in the new contact ID list.</p>
    pub fn new_contact_ids(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.new_contact_ids.as_deref()
    }
    /// <p>The date and time when the override goes into effect.</p>
    pub fn start_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.start_time.as_ref()
    }
    /// <p>The date and time when the override ends.</p>
    pub fn end_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.end_time.as_ref()
    }
    /// <p>A token that ensures that the operation is called only once with the specified details.</p>
    pub fn idempotency_token(&self) -> ::std::option::Option<&str> {
        self.idempotency_token.as_deref()
    }
}
impl CreateRotationOverrideInput {
    /// Creates a new builder-style object to manufacture [`CreateRotationOverrideInput`](crate::operation::create_rotation_override::CreateRotationOverrideInput).
    pub fn builder(
    ) -> crate::operation::create_rotation_override::builders::CreateRotationOverrideInputBuilder
    {
        crate::operation::create_rotation_override::builders::CreateRotationOverrideInputBuilder::default()
    }
}

/// A builder for [`CreateRotationOverrideInput`](crate::operation::create_rotation_override::CreateRotationOverrideInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateRotationOverrideInputBuilder {
    pub(crate) rotation_id: ::std::option::Option<::std::string::String>,
    pub(crate) new_contact_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) start_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) end_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) idempotency_token: ::std::option::Option<::std::string::String>,
}
impl CreateRotationOverrideInputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the rotation to create an override for.</p>
    pub fn rotation_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.rotation_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the rotation to create an override for.</p>
    pub fn set_rotation_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.rotation_id = input;
        self
    }
    /// Appends an item to `new_contact_ids`.
    ///
    /// To override the contents of this collection use [`set_new_contact_ids`](Self::set_new_contact_ids).
    ///
    /// <p>The Amazon Resource Names (ARNs) of the contacts to replace those in the current on-call rotation with.</p>
    /// <p>If you want to include any current team members in the override shift, you must include their ARNs in the new contact ID list.</p>
    pub fn new_contact_ids(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.new_contact_ids.unwrap_or_default();
        v.push(input.into());
        self.new_contact_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>The Amazon Resource Names (ARNs) of the contacts to replace those in the current on-call rotation with.</p>
    /// <p>If you want to include any current team members in the override shift, you must include their ARNs in the new contact ID list.</p>
    pub fn set_new_contact_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.new_contact_ids = input;
        self
    }
    /// <p>The date and time when the override goes into effect.</p>
    pub fn start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.start_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time when the override goes into effect.</p>
    pub fn set_start_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.start_time = input;
        self
    }
    /// <p>The date and time when the override ends.</p>
    pub fn end_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.end_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time when the override ends.</p>
    pub fn set_end_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.end_time = input;
        self
    }
    /// <p>A token that ensures that the operation is called only once with the specified details.</p>
    pub fn idempotency_token(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.idempotency_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A token that ensures that the operation is called only once with the specified details.</p>
    pub fn set_idempotency_token(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.idempotency_token = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateRotationOverrideInput`](crate::operation::create_rotation_override::CreateRotationOverrideInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_rotation_override::CreateRotationOverrideInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::create_rotation_override::CreateRotationOverrideInput {
                rotation_id: self.rotation_id,
                new_contact_ids: self.new_contact_ids,
                start_time: self.start_time,
                end_time: self.end_time,
                idempotency_token: self.idempotency_token,
            },
        )
    }
}
