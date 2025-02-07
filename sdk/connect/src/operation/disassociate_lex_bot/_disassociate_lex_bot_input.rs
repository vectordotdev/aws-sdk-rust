// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DisassociateLexBotInput {
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    #[doc(hidden)]
    pub instance_id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the Amazon Lex bot. Maximum character limit of 50.</p>
    #[doc(hidden)]
    pub bot_name: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Web Services Region in which the Amazon Lex bot has been created.</p>
    #[doc(hidden)]
    pub lex_region: ::std::option::Option<::std::string::String>,
}
impl DisassociateLexBotInput {
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn instance_id(&self) -> ::std::option::Option<&str> {
        self.instance_id.as_deref()
    }
    /// <p>The name of the Amazon Lex bot. Maximum character limit of 50.</p>
    pub fn bot_name(&self) -> ::std::option::Option<&str> {
        self.bot_name.as_deref()
    }
    /// <p>The Amazon Web Services Region in which the Amazon Lex bot has been created.</p>
    pub fn lex_region(&self) -> ::std::option::Option<&str> {
        self.lex_region.as_deref()
    }
}
impl DisassociateLexBotInput {
    /// Creates a new builder-style object to manufacture [`DisassociateLexBotInput`](crate::operation::disassociate_lex_bot::DisassociateLexBotInput).
    pub fn builder(
    ) -> crate::operation::disassociate_lex_bot::builders::DisassociateLexBotInputBuilder {
        crate::operation::disassociate_lex_bot::builders::DisassociateLexBotInputBuilder::default()
    }
}

/// A builder for [`DisassociateLexBotInput`](crate::operation::disassociate_lex_bot::DisassociateLexBotInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DisassociateLexBotInputBuilder {
    pub(crate) instance_id: ::std::option::Option<::std::string::String>,
    pub(crate) bot_name: ::std::option::Option<::std::string::String>,
    pub(crate) lex_region: ::std::option::Option<::std::string::String>,
}
impl DisassociateLexBotInputBuilder {
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.instance_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.instance_id = input;
        self
    }
    /// <p>The name of the Amazon Lex bot. Maximum character limit of 50.</p>
    pub fn bot_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bot_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the Amazon Lex bot. Maximum character limit of 50.</p>
    pub fn set_bot_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bot_name = input;
        self
    }
    /// <p>The Amazon Web Services Region in which the Amazon Lex bot has been created.</p>
    pub fn lex_region(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.lex_region = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Web Services Region in which the Amazon Lex bot has been created.</p>
    pub fn set_lex_region(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.lex_region = input;
        self
    }
    /// Consumes the builder and constructs a [`DisassociateLexBotInput`](crate::operation::disassociate_lex_bot::DisassociateLexBotInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::disassociate_lex_bot::DisassociateLexBotInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::disassociate_lex_bot::DisassociateLexBotInput {
                instance_id: self.instance_id,
                bot_name: self.bot_name,
                lex_region: self.lex_region,
            },
        )
    }
}
