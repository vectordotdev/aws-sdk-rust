// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Description of the VPC connection.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct VpcConnectionInfo {
    /// <p>The Amazon Resource Name (ARN) of the VPC connection.</p>
    #[doc(hidden)]
    pub vpc_connection_arn: ::std::option::Option<::std::string::String>,
    /// <p>The owner of the VPC Connection.</p>
    #[doc(hidden)]
    pub owner: ::std::option::Option<::std::string::String>,
    /// <p>Description of the requester that calls the API operation.</p>
    #[doc(hidden)]
    pub user_identity: ::std::option::Option<crate::types::UserIdentity>,
    /// <p>The time when Amazon MSK creates the VPC Connnection.</p>
    #[doc(hidden)]
    pub creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl VpcConnectionInfo {
    /// <p>The Amazon Resource Name (ARN) of the VPC connection.</p>
    pub fn vpc_connection_arn(&self) -> ::std::option::Option<&str> {
        self.vpc_connection_arn.as_deref()
    }
    /// <p>The owner of the VPC Connection.</p>
    pub fn owner(&self) -> ::std::option::Option<&str> {
        self.owner.as_deref()
    }
    /// <p>Description of the requester that calls the API operation.</p>
    pub fn user_identity(&self) -> ::std::option::Option<&crate::types::UserIdentity> {
        self.user_identity.as_ref()
    }
    /// <p>The time when Amazon MSK creates the VPC Connnection.</p>
    pub fn creation_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.creation_time.as_ref()
    }
}
impl VpcConnectionInfo {
    /// Creates a new builder-style object to manufacture [`VpcConnectionInfo`](crate::types::VpcConnectionInfo).
    pub fn builder() -> crate::types::builders::VpcConnectionInfoBuilder {
        crate::types::builders::VpcConnectionInfoBuilder::default()
    }
}

/// A builder for [`VpcConnectionInfo`](crate::types::VpcConnectionInfo).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct VpcConnectionInfoBuilder {
    pub(crate) vpc_connection_arn: ::std::option::Option<::std::string::String>,
    pub(crate) owner: ::std::option::Option<::std::string::String>,
    pub(crate) user_identity: ::std::option::Option<crate::types::UserIdentity>,
    pub(crate) creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl VpcConnectionInfoBuilder {
    /// <p>The Amazon Resource Name (ARN) of the VPC connection.</p>
    pub fn vpc_connection_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.vpc_connection_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the VPC connection.</p>
    pub fn set_vpc_connection_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.vpc_connection_arn = input;
        self
    }
    /// <p>The owner of the VPC Connection.</p>
    pub fn owner(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.owner = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The owner of the VPC Connection.</p>
    pub fn set_owner(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.owner = input;
        self
    }
    /// <p>Description of the requester that calls the API operation.</p>
    pub fn user_identity(mut self, input: crate::types::UserIdentity) -> Self {
        self.user_identity = ::std::option::Option::Some(input);
        self
    }
    /// <p>Description of the requester that calls the API operation.</p>
    pub fn set_user_identity(
        mut self,
        input: ::std::option::Option<crate::types::UserIdentity>,
    ) -> Self {
        self.user_identity = input;
        self
    }
    /// <p>The time when Amazon MSK creates the VPC Connnection.</p>
    pub fn creation_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.creation_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time when Amazon MSK creates the VPC Connnection.</p>
    pub fn set_creation_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.creation_time = input;
        self
    }
    /// Consumes the builder and constructs a [`VpcConnectionInfo`](crate::types::VpcConnectionInfo).
    pub fn build(self) -> crate::types::VpcConnectionInfo {
        crate::types::VpcConnectionInfo {
            vpc_connection_arn: self.vpc_connection_arn,
            owner: self.owner,
            user_identity: self.user_identity,
            creation_time: self.creation_time,
        }
    }
}
