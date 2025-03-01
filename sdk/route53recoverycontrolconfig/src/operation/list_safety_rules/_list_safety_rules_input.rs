// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListSafetyRulesInput {
    /// <p>The Amazon Resource Name (ARN) of the control panel.</p>
    #[doc(hidden)]
    pub control_panel_arn: ::std::option::Option<::std::string::String>,
    /// <p>The number of objects that you want to return with this call.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
    /// <p>The token that identifies which batch of results you want to see.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl ListSafetyRulesInput {
    /// <p>The Amazon Resource Name (ARN) of the control panel.</p>
    pub fn control_panel_arn(&self) -> ::std::option::Option<&str> {
        self.control_panel_arn.as_deref()
    }
    /// <p>The number of objects that you want to return with this call.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>The token that identifies which batch of results you want to see.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ListSafetyRulesInput {
    /// Creates a new builder-style object to manufacture [`ListSafetyRulesInput`](crate::operation::list_safety_rules::ListSafetyRulesInput).
    pub fn builder() -> crate::operation::list_safety_rules::builders::ListSafetyRulesInputBuilder {
        crate::operation::list_safety_rules::builders::ListSafetyRulesInputBuilder::default()
    }
}

/// A builder for [`ListSafetyRulesInput`](crate::operation::list_safety_rules::ListSafetyRulesInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListSafetyRulesInputBuilder {
    pub(crate) control_panel_arn: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl ListSafetyRulesInputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the control panel.</p>
    pub fn control_panel_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.control_panel_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the control panel.</p>
    pub fn set_control_panel_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.control_panel_arn = input;
        self
    }
    /// <p>The number of objects that you want to return with this call.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of objects that you want to return with this call.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The token that identifies which batch of results you want to see.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token that identifies which batch of results you want to see.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Consumes the builder and constructs a [`ListSafetyRulesInput`](crate::operation::list_safety_rules::ListSafetyRulesInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_safety_rules::ListSafetyRulesInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::list_safety_rules::ListSafetyRulesInput {
            control_panel_arn: self.control_panel_arn,
            max_results: self.max_results,
            next_token: self.next_token,
        })
    }
}
