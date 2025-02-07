// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetRoomInput {
    /// <p>Identifier of the room for which the configuration is to be retrieved. Currently this must be an ARN.</p>
    #[doc(hidden)]
    pub identifier: ::std::option::Option<::std::string::String>,
}
impl GetRoomInput {
    /// <p>Identifier of the room for which the configuration is to be retrieved. Currently this must be an ARN.</p>
    pub fn identifier(&self) -> ::std::option::Option<&str> {
        self.identifier.as_deref()
    }
}
impl GetRoomInput {
    /// Creates a new builder-style object to manufacture [`GetRoomInput`](crate::operation::get_room::GetRoomInput).
    pub fn builder() -> crate::operation::get_room::builders::GetRoomInputBuilder {
        crate::operation::get_room::builders::GetRoomInputBuilder::default()
    }
}

/// A builder for [`GetRoomInput`](crate::operation::get_room::GetRoomInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetRoomInputBuilder {
    pub(crate) identifier: ::std::option::Option<::std::string::String>,
}
impl GetRoomInputBuilder {
    /// <p>Identifier of the room for which the configuration is to be retrieved. Currently this must be an ARN.</p>
    pub fn identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Identifier of the room for which the configuration is to be retrieved. Currently this must be an ARN.</p>
    pub fn set_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.identifier = input;
        self
    }
    /// Consumes the builder and constructs a [`GetRoomInput`](crate::operation::get_room::GetRoomInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_room::GetRoomInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_room::GetRoomInput {
            identifier: self.identifier,
        })
    }
}
