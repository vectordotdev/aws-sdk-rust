// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Parameters to define a mitigation action that adds a blank policy to restrict permissions.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ReplaceDefaultPolicyVersionParams {
    /// <p>The name of the template to be applied. The only supported value is <code>BLANK_POLICY</code>.</p>
    #[doc(hidden)]
    pub template_name: ::std::option::Option<crate::types::PolicyTemplateName>,
}
impl ReplaceDefaultPolicyVersionParams {
    /// <p>The name of the template to be applied. The only supported value is <code>BLANK_POLICY</code>.</p>
    pub fn template_name(&self) -> ::std::option::Option<&crate::types::PolicyTemplateName> {
        self.template_name.as_ref()
    }
}
impl ReplaceDefaultPolicyVersionParams {
    /// Creates a new builder-style object to manufacture [`ReplaceDefaultPolicyVersionParams`](crate::types::ReplaceDefaultPolicyVersionParams).
    pub fn builder() -> crate::types::builders::ReplaceDefaultPolicyVersionParamsBuilder {
        crate::types::builders::ReplaceDefaultPolicyVersionParamsBuilder::default()
    }
}

/// A builder for [`ReplaceDefaultPolicyVersionParams`](crate::types::ReplaceDefaultPolicyVersionParams).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ReplaceDefaultPolicyVersionParamsBuilder {
    pub(crate) template_name: ::std::option::Option<crate::types::PolicyTemplateName>,
}
impl ReplaceDefaultPolicyVersionParamsBuilder {
    /// <p>The name of the template to be applied. The only supported value is <code>BLANK_POLICY</code>.</p>
    pub fn template_name(mut self, input: crate::types::PolicyTemplateName) -> Self {
        self.template_name = ::std::option::Option::Some(input);
        self
    }
    /// <p>The name of the template to be applied. The only supported value is <code>BLANK_POLICY</code>.</p>
    pub fn set_template_name(
        mut self,
        input: ::std::option::Option<crate::types::PolicyTemplateName>,
    ) -> Self {
        self.template_name = input;
        self
    }
    /// Consumes the builder and constructs a [`ReplaceDefaultPolicyVersionParams`](crate::types::ReplaceDefaultPolicyVersionParams).
    pub fn build(self) -> crate::types::ReplaceDefaultPolicyVersionParams {
        crate::types::ReplaceDefaultPolicyVersionParams {
            template_name: self.template_name,
        }
    }
}
