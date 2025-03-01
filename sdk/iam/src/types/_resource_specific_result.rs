// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the result of the simulation of a single API operation call on a single resource.</p>
/// <p>This data type is used by a member of the <code>EvaluationResult</code> data type.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ResourceSpecificResult {
    /// <p>The name of the simulated resource, in Amazon Resource Name (ARN) format.</p>
    #[doc(hidden)]
    pub eval_resource_name: ::std::option::Option<::std::string::String>,
    /// <p>The result of the simulation of the simulated API operation on the resource specified in <code>EvalResourceName</code>.</p>
    #[doc(hidden)]
    pub eval_resource_decision: ::std::option::Option<crate::types::PolicyEvaluationDecisionType>,
    /// <p>A list of the statements in the input policies that determine the result for this part of the simulation. Remember that even if multiple statements allow the operation on the resource, if <i>any</i> statement denies that operation, then the explicit deny overrides any allow. In addition, the deny statement is the only entry included in the result.</p>
    #[doc(hidden)]
    pub matched_statements: ::std::option::Option<::std::vec::Vec<crate::types::Statement>>,
    /// <p>A list of context keys that are required by the included input policies but that were not provided by one of the input parameters. This list is used when a list of ARNs is included in the <code>ResourceArns</code> parameter instead of "*". If you do not specify individual resources, by setting <code>ResourceArns</code> to "*" or by not including the <code>ResourceArns</code> parameter, then any missing context values are instead included under the <code>EvaluationResults</code> section. To discover the context keys used by a set of policies, you can call <code>GetContextKeysForCustomPolicy</code> or <code>GetContextKeysForPrincipalPolicy</code>.</p>
    #[doc(hidden)]
    pub missing_context_values: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Additional details about the results of the evaluation decision on a single resource. This parameter is returned only for cross-account simulations. This parameter explains how each policy type contributes to the resource-specific evaluation decision.</p>
    #[doc(hidden)]
    pub eval_decision_details: ::std::option::Option<
        ::std::collections::HashMap<
            ::std::string::String,
            crate::types::PolicyEvaluationDecisionType,
        >,
    >,
    /// <p>Contains information about the effect that a permissions boundary has on a policy simulation when that boundary is applied to an IAM entity.</p>
    #[doc(hidden)]
    pub permissions_boundary_decision_detail:
        ::std::option::Option<crate::types::PermissionsBoundaryDecisionDetail>,
}
impl ResourceSpecificResult {
    /// <p>The name of the simulated resource, in Amazon Resource Name (ARN) format.</p>
    pub fn eval_resource_name(&self) -> ::std::option::Option<&str> {
        self.eval_resource_name.as_deref()
    }
    /// <p>The result of the simulation of the simulated API operation on the resource specified in <code>EvalResourceName</code>.</p>
    pub fn eval_resource_decision(
        &self,
    ) -> ::std::option::Option<&crate::types::PolicyEvaluationDecisionType> {
        self.eval_resource_decision.as_ref()
    }
    /// <p>A list of the statements in the input policies that determine the result for this part of the simulation. Remember that even if multiple statements allow the operation on the resource, if <i>any</i> statement denies that operation, then the explicit deny overrides any allow. In addition, the deny statement is the only entry included in the result.</p>
    pub fn matched_statements(&self) -> ::std::option::Option<&[crate::types::Statement]> {
        self.matched_statements.as_deref()
    }
    /// <p>A list of context keys that are required by the included input policies but that were not provided by one of the input parameters. This list is used when a list of ARNs is included in the <code>ResourceArns</code> parameter instead of "*". If you do not specify individual resources, by setting <code>ResourceArns</code> to "*" or by not including the <code>ResourceArns</code> parameter, then any missing context values are instead included under the <code>EvaluationResults</code> section. To discover the context keys used by a set of policies, you can call <code>GetContextKeysForCustomPolicy</code> or <code>GetContextKeysForPrincipalPolicy</code>.</p>
    pub fn missing_context_values(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.missing_context_values.as_deref()
    }
    /// <p>Additional details about the results of the evaluation decision on a single resource. This parameter is returned only for cross-account simulations. This parameter explains how each policy type contributes to the resource-specific evaluation decision.</p>
    pub fn eval_decision_details(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<
            ::std::string::String,
            crate::types::PolicyEvaluationDecisionType,
        >,
    > {
        self.eval_decision_details.as_ref()
    }
    /// <p>Contains information about the effect that a permissions boundary has on a policy simulation when that boundary is applied to an IAM entity.</p>
    pub fn permissions_boundary_decision_detail(
        &self,
    ) -> ::std::option::Option<&crate::types::PermissionsBoundaryDecisionDetail> {
        self.permissions_boundary_decision_detail.as_ref()
    }
}
impl ResourceSpecificResult {
    /// Creates a new builder-style object to manufacture [`ResourceSpecificResult`](crate::types::ResourceSpecificResult).
    pub fn builder() -> crate::types::builders::ResourceSpecificResultBuilder {
        crate::types::builders::ResourceSpecificResultBuilder::default()
    }
}

/// A builder for [`ResourceSpecificResult`](crate::types::ResourceSpecificResult).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ResourceSpecificResultBuilder {
    pub(crate) eval_resource_name: ::std::option::Option<::std::string::String>,
    pub(crate) eval_resource_decision:
        ::std::option::Option<crate::types::PolicyEvaluationDecisionType>,
    pub(crate) matched_statements: ::std::option::Option<::std::vec::Vec<crate::types::Statement>>,
    pub(crate) missing_context_values:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) eval_decision_details: ::std::option::Option<
        ::std::collections::HashMap<
            ::std::string::String,
            crate::types::PolicyEvaluationDecisionType,
        >,
    >,
    pub(crate) permissions_boundary_decision_detail:
        ::std::option::Option<crate::types::PermissionsBoundaryDecisionDetail>,
}
impl ResourceSpecificResultBuilder {
    /// <p>The name of the simulated resource, in Amazon Resource Name (ARN) format.</p>
    pub fn eval_resource_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.eval_resource_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the simulated resource, in Amazon Resource Name (ARN) format.</p>
    pub fn set_eval_resource_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.eval_resource_name = input;
        self
    }
    /// <p>The result of the simulation of the simulated API operation on the resource specified in <code>EvalResourceName</code>.</p>
    pub fn eval_resource_decision(
        mut self,
        input: crate::types::PolicyEvaluationDecisionType,
    ) -> Self {
        self.eval_resource_decision = ::std::option::Option::Some(input);
        self
    }
    /// <p>The result of the simulation of the simulated API operation on the resource specified in <code>EvalResourceName</code>.</p>
    pub fn set_eval_resource_decision(
        mut self,
        input: ::std::option::Option<crate::types::PolicyEvaluationDecisionType>,
    ) -> Self {
        self.eval_resource_decision = input;
        self
    }
    /// Appends an item to `matched_statements`.
    ///
    /// To override the contents of this collection use [`set_matched_statements`](Self::set_matched_statements).
    ///
    /// <p>A list of the statements in the input policies that determine the result for this part of the simulation. Remember that even if multiple statements allow the operation on the resource, if <i>any</i> statement denies that operation, then the explicit deny overrides any allow. In addition, the deny statement is the only entry included in the result.</p>
    pub fn matched_statements(mut self, input: crate::types::Statement) -> Self {
        let mut v = self.matched_statements.unwrap_or_default();
        v.push(input);
        self.matched_statements = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of the statements in the input policies that determine the result for this part of the simulation. Remember that even if multiple statements allow the operation on the resource, if <i>any</i> statement denies that operation, then the explicit deny overrides any allow. In addition, the deny statement is the only entry included in the result.</p>
    pub fn set_matched_statements(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Statement>>,
    ) -> Self {
        self.matched_statements = input;
        self
    }
    /// Appends an item to `missing_context_values`.
    ///
    /// To override the contents of this collection use [`set_missing_context_values`](Self::set_missing_context_values).
    ///
    /// <p>A list of context keys that are required by the included input policies but that were not provided by one of the input parameters. This list is used when a list of ARNs is included in the <code>ResourceArns</code> parameter instead of "*". If you do not specify individual resources, by setting <code>ResourceArns</code> to "*" or by not including the <code>ResourceArns</code> parameter, then any missing context values are instead included under the <code>EvaluationResults</code> section. To discover the context keys used by a set of policies, you can call <code>GetContextKeysForCustomPolicy</code> or <code>GetContextKeysForPrincipalPolicy</code>.</p>
    pub fn missing_context_values(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.missing_context_values.unwrap_or_default();
        v.push(input.into());
        self.missing_context_values = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of context keys that are required by the included input policies but that were not provided by one of the input parameters. This list is used when a list of ARNs is included in the <code>ResourceArns</code> parameter instead of "*". If you do not specify individual resources, by setting <code>ResourceArns</code> to "*" or by not including the <code>ResourceArns</code> parameter, then any missing context values are instead included under the <code>EvaluationResults</code> section. To discover the context keys used by a set of policies, you can call <code>GetContextKeysForCustomPolicy</code> or <code>GetContextKeysForPrincipalPolicy</code>.</p>
    pub fn set_missing_context_values(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.missing_context_values = input;
        self
    }
    /// Adds a key-value pair to `eval_decision_details`.
    ///
    /// To override the contents of this collection use [`set_eval_decision_details`](Self::set_eval_decision_details).
    ///
    /// <p>Additional details about the results of the evaluation decision on a single resource. This parameter is returned only for cross-account simulations. This parameter explains how each policy type contributes to the resource-specific evaluation decision.</p>
    pub fn eval_decision_details(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: crate::types::PolicyEvaluationDecisionType,
    ) -> Self {
        let mut hash_map = self.eval_decision_details.unwrap_or_default();
        hash_map.insert(k.into(), v);
        self.eval_decision_details = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>Additional details about the results of the evaluation decision on a single resource. This parameter is returned only for cross-account simulations. This parameter explains how each policy type contributes to the resource-specific evaluation decision.</p>
    pub fn set_eval_decision_details(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<
                ::std::string::String,
                crate::types::PolicyEvaluationDecisionType,
            >,
        >,
    ) -> Self {
        self.eval_decision_details = input;
        self
    }
    /// <p>Contains information about the effect that a permissions boundary has on a policy simulation when that boundary is applied to an IAM entity.</p>
    pub fn permissions_boundary_decision_detail(
        mut self,
        input: crate::types::PermissionsBoundaryDecisionDetail,
    ) -> Self {
        self.permissions_boundary_decision_detail = ::std::option::Option::Some(input);
        self
    }
    /// <p>Contains information about the effect that a permissions boundary has on a policy simulation when that boundary is applied to an IAM entity.</p>
    pub fn set_permissions_boundary_decision_detail(
        mut self,
        input: ::std::option::Option<crate::types::PermissionsBoundaryDecisionDetail>,
    ) -> Self {
        self.permissions_boundary_decision_detail = input;
        self
    }
    /// Consumes the builder and constructs a [`ResourceSpecificResult`](crate::types::ResourceSpecificResult).
    pub fn build(self) -> crate::types::ResourceSpecificResult {
        crate::types::ResourceSpecificResult {
            eval_resource_name: self.eval_resource_name,
            eval_resource_decision: self.eval_resource_decision,
            matched_statements: self.matched_statements,
            missing_context_values: self.missing_context_values,
            eval_decision_details: self.eval_decision_details,
            permissions_boundary_decision_detail: self.permissions_boundary_decision_detail,
        }
    }
}
