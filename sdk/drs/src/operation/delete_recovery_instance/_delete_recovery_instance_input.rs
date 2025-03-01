// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteRecoveryInstanceInput {
    /// <p>The ID of the Recovery Instance to be deleted.</p>
    #[doc(hidden)]
    pub recovery_instance_id: ::std::option::Option<::std::string::String>,
}
impl DeleteRecoveryInstanceInput {
    /// <p>The ID of the Recovery Instance to be deleted.</p>
    pub fn recovery_instance_id(&self) -> ::std::option::Option<&str> {
        self.recovery_instance_id.as_deref()
    }
}
impl DeleteRecoveryInstanceInput {
    /// Creates a new builder-style object to manufacture [`DeleteRecoveryInstanceInput`](crate::operation::delete_recovery_instance::DeleteRecoveryInstanceInput).
    pub fn builder(
    ) -> crate::operation::delete_recovery_instance::builders::DeleteRecoveryInstanceInputBuilder
    {
        crate::operation::delete_recovery_instance::builders::DeleteRecoveryInstanceInputBuilder::default()
    }
}

/// A builder for [`DeleteRecoveryInstanceInput`](crate::operation::delete_recovery_instance::DeleteRecoveryInstanceInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteRecoveryInstanceInputBuilder {
    pub(crate) recovery_instance_id: ::std::option::Option<::std::string::String>,
}
impl DeleteRecoveryInstanceInputBuilder {
    /// <p>The ID of the Recovery Instance to be deleted.</p>
    pub fn recovery_instance_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.recovery_instance_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Recovery Instance to be deleted.</p>
    pub fn set_recovery_instance_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.recovery_instance_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DeleteRecoveryInstanceInput`](crate::operation::delete_recovery_instance::DeleteRecoveryInstanceInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_recovery_instance::DeleteRecoveryInstanceInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::delete_recovery_instance::DeleteRecoveryInstanceInput {
                recovery_instance_id: self.recovery_instance_id,
            },
        )
    }
}
