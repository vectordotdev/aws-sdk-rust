// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the parameters for CreateAppCookieStickinessPolicy.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateAppCookieStickinessPolicyInput {
    /// <p>The name of the load balancer.</p>
    #[doc(hidden)]
    pub load_balancer_name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the policy being created. Policy names must consist of alphanumeric characters and dashes (-). This name must be unique within the set of policies for this load balancer.</p>
    #[doc(hidden)]
    pub policy_name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the application cookie used for stickiness.</p>
    #[doc(hidden)]
    pub cookie_name: ::std::option::Option<::std::string::String>,
}
impl CreateAppCookieStickinessPolicyInput {
    /// <p>The name of the load balancer.</p>
    pub fn load_balancer_name(&self) -> ::std::option::Option<&str> {
        self.load_balancer_name.as_deref()
    }
    /// <p>The name of the policy being created. Policy names must consist of alphanumeric characters and dashes (-). This name must be unique within the set of policies for this load balancer.</p>
    pub fn policy_name(&self) -> ::std::option::Option<&str> {
        self.policy_name.as_deref()
    }
    /// <p>The name of the application cookie used for stickiness.</p>
    pub fn cookie_name(&self) -> ::std::option::Option<&str> {
        self.cookie_name.as_deref()
    }
}
impl CreateAppCookieStickinessPolicyInput {
    /// Creates a new builder-style object to manufacture [`CreateAppCookieStickinessPolicyInput`](crate::operation::create_app_cookie_stickiness_policy::CreateAppCookieStickinessPolicyInput).
    pub fn builder() -> crate::operation::create_app_cookie_stickiness_policy::builders::CreateAppCookieStickinessPolicyInputBuilder{
        crate::operation::create_app_cookie_stickiness_policy::builders::CreateAppCookieStickinessPolicyInputBuilder::default()
    }
}

/// A builder for [`CreateAppCookieStickinessPolicyInput`](crate::operation::create_app_cookie_stickiness_policy::CreateAppCookieStickinessPolicyInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateAppCookieStickinessPolicyInputBuilder {
    pub(crate) load_balancer_name: ::std::option::Option<::std::string::String>,
    pub(crate) policy_name: ::std::option::Option<::std::string::String>,
    pub(crate) cookie_name: ::std::option::Option<::std::string::String>,
}
impl CreateAppCookieStickinessPolicyInputBuilder {
    /// <p>The name of the load balancer.</p>
    pub fn load_balancer_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.load_balancer_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the load balancer.</p>
    pub fn set_load_balancer_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.load_balancer_name = input;
        self
    }
    /// <p>The name of the policy being created. Policy names must consist of alphanumeric characters and dashes (-). This name must be unique within the set of policies for this load balancer.</p>
    pub fn policy_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.policy_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the policy being created. Policy names must consist of alphanumeric characters and dashes (-). This name must be unique within the set of policies for this load balancer.</p>
    pub fn set_policy_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.policy_name = input;
        self
    }
    /// <p>The name of the application cookie used for stickiness.</p>
    pub fn cookie_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.cookie_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the application cookie used for stickiness.</p>
    pub fn set_cookie_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.cookie_name = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateAppCookieStickinessPolicyInput`](crate::operation::create_app_cookie_stickiness_policy::CreateAppCookieStickinessPolicyInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_app_cookie_stickiness_policy::CreateAppCookieStickinessPolicyInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::create_app_cookie_stickiness_policy::CreateAppCookieStickinessPolicyInput {
                load_balancer_name: self.load_balancer_name
                ,
                policy_name: self.policy_name
                ,
                cookie_name: self.cookie_name
                ,
            }
        )
    }
}
