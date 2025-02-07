// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateVpcAttachmentInput {
    /// <p>The ID of a core network for the VPC attachment.</p>
    #[doc(hidden)]
    pub core_network_id: ::std::option::Option<::std::string::String>,
    /// <p>The ARN of the VPC.</p>
    #[doc(hidden)]
    pub vpc_arn: ::std::option::Option<::std::string::String>,
    /// <p>The subnet ARN of the VPC attachment.</p>
    #[doc(hidden)]
    pub subnet_arns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Options for the VPC attachment.</p>
    #[doc(hidden)]
    pub options: ::std::option::Option<crate::types::VpcOptions>,
    /// <p>The key-value tags associated with the request.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    /// <p>The client token associated with the request.</p>
    #[doc(hidden)]
    pub client_token: ::std::option::Option<::std::string::String>,
}
impl CreateVpcAttachmentInput {
    /// <p>The ID of a core network for the VPC attachment.</p>
    pub fn core_network_id(&self) -> ::std::option::Option<&str> {
        self.core_network_id.as_deref()
    }
    /// <p>The ARN of the VPC.</p>
    pub fn vpc_arn(&self) -> ::std::option::Option<&str> {
        self.vpc_arn.as_deref()
    }
    /// <p>The subnet ARN of the VPC attachment.</p>
    pub fn subnet_arns(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.subnet_arns.as_deref()
    }
    /// <p>Options for the VPC attachment.</p>
    pub fn options(&self) -> ::std::option::Option<&crate::types::VpcOptions> {
        self.options.as_ref()
    }
    /// <p>The key-value tags associated with the request.</p>
    pub fn tags(&self) -> ::std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
    /// <p>The client token associated with the request.</p>
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }
}
impl CreateVpcAttachmentInput {
    /// Creates a new builder-style object to manufacture [`CreateVpcAttachmentInput`](crate::operation::create_vpc_attachment::CreateVpcAttachmentInput).
    pub fn builder(
    ) -> crate::operation::create_vpc_attachment::builders::CreateVpcAttachmentInputBuilder {
        crate::operation::create_vpc_attachment::builders::CreateVpcAttachmentInputBuilder::default(
        )
    }
}

/// A builder for [`CreateVpcAttachmentInput`](crate::operation::create_vpc_attachment::CreateVpcAttachmentInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateVpcAttachmentInputBuilder {
    pub(crate) core_network_id: ::std::option::Option<::std::string::String>,
    pub(crate) vpc_arn: ::std::option::Option<::std::string::String>,
    pub(crate) subnet_arns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) options: ::std::option::Option<crate::types::VpcOptions>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
}
impl CreateVpcAttachmentInputBuilder {
    /// <p>The ID of a core network for the VPC attachment.</p>
    pub fn core_network_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.core_network_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of a core network for the VPC attachment.</p>
    pub fn set_core_network_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.core_network_id = input;
        self
    }
    /// <p>The ARN of the VPC.</p>
    pub fn vpc_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.vpc_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the VPC.</p>
    pub fn set_vpc_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.vpc_arn = input;
        self
    }
    /// Appends an item to `subnet_arns`.
    ///
    /// To override the contents of this collection use [`set_subnet_arns`](Self::set_subnet_arns).
    ///
    /// <p>The subnet ARN of the VPC attachment.</p>
    pub fn subnet_arns(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.subnet_arns.unwrap_or_default();
        v.push(input.into());
        self.subnet_arns = ::std::option::Option::Some(v);
        self
    }
    /// <p>The subnet ARN of the VPC attachment.</p>
    pub fn set_subnet_arns(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.subnet_arns = input;
        self
    }
    /// <p>Options for the VPC attachment.</p>
    pub fn options(mut self, input: crate::types::VpcOptions) -> Self {
        self.options = ::std::option::Option::Some(input);
        self
    }
    /// <p>Options for the VPC attachment.</p>
    pub fn set_options(mut self, input: ::std::option::Option<crate::types::VpcOptions>) -> Self {
        self.options = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The key-value tags associated with the request.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>The key-value tags associated with the request.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// <p>The client token associated with the request.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The client token associated with the request.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateVpcAttachmentInput`](crate::operation::create_vpc_attachment::CreateVpcAttachmentInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_vpc_attachment::CreateVpcAttachmentInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::create_vpc_attachment::CreateVpcAttachmentInput {
                core_network_id: self.core_network_id,
                vpc_arn: self.vpc_arn,
                subnet_arns: self.subnet_arns,
                options: self.options,
                tags: self.tags,
                client_token: self.client_token,
            },
        )
    }
}
