// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents a request to set the position of a receipt rule in a receipt rule set. You use receipt rule sets to receive email with Amazon SES. For more information, see the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/receiving-email-concepts.html">Amazon SES Developer Guide</a>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SetReceiptRulePositionInput {
    /// <p>The name of the receipt rule set that contains the receipt rule to reposition.</p>
    #[doc(hidden)]
    pub rule_set_name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the receipt rule to reposition.</p>
    #[doc(hidden)]
    pub rule_name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the receipt rule after which to place the specified receipt rule.</p>
    #[doc(hidden)]
    pub after: ::std::option::Option<::std::string::String>,
}
impl SetReceiptRulePositionInput {
    /// <p>The name of the receipt rule set that contains the receipt rule to reposition.</p>
    pub fn rule_set_name(&self) -> ::std::option::Option<&str> {
        self.rule_set_name.as_deref()
    }
    /// <p>The name of the receipt rule to reposition.</p>
    pub fn rule_name(&self) -> ::std::option::Option<&str> {
        self.rule_name.as_deref()
    }
    /// <p>The name of the receipt rule after which to place the specified receipt rule.</p>
    pub fn after(&self) -> ::std::option::Option<&str> {
        self.after.as_deref()
    }
}
impl SetReceiptRulePositionInput {
    /// Creates a new builder-style object to manufacture [`SetReceiptRulePositionInput`](crate::operation::set_receipt_rule_position::SetReceiptRulePositionInput).
    pub fn builder(
    ) -> crate::operation::set_receipt_rule_position::builders::SetReceiptRulePositionInputBuilder
    {
        crate::operation::set_receipt_rule_position::builders::SetReceiptRulePositionInputBuilder::default()
    }
}

/// A builder for [`SetReceiptRulePositionInput`](crate::operation::set_receipt_rule_position::SetReceiptRulePositionInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SetReceiptRulePositionInputBuilder {
    pub(crate) rule_set_name: ::std::option::Option<::std::string::String>,
    pub(crate) rule_name: ::std::option::Option<::std::string::String>,
    pub(crate) after: ::std::option::Option<::std::string::String>,
}
impl SetReceiptRulePositionInputBuilder {
    /// <p>The name of the receipt rule set that contains the receipt rule to reposition.</p>
    pub fn rule_set_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.rule_set_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the receipt rule set that contains the receipt rule to reposition.</p>
    pub fn set_rule_set_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.rule_set_name = input;
        self
    }
    /// <p>The name of the receipt rule to reposition.</p>
    pub fn rule_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.rule_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the receipt rule to reposition.</p>
    pub fn set_rule_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.rule_name = input;
        self
    }
    /// <p>The name of the receipt rule after which to place the specified receipt rule.</p>
    pub fn after(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.after = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the receipt rule after which to place the specified receipt rule.</p>
    pub fn set_after(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.after = input;
        self
    }
    /// Consumes the builder and constructs a [`SetReceiptRulePositionInput`](crate::operation::set_receipt_rule_position::SetReceiptRulePositionInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::set_receipt_rule_position::SetReceiptRulePositionInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::set_receipt_rule_position::SetReceiptRulePositionInput {
                rule_set_name: self.rule_set_name,
                rule_name: self.rule_name,
                after: self.after,
            },
        )
    }
}
