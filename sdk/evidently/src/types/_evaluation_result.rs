// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>This structure displays the results of one feature evaluation assignment to one user session.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EvaluationResult {
    /// <p>The name or ARN of the project that contains the feature being evaluated.</p>
    #[doc(hidden)]
    pub project: ::std::option::Option<::std::string::String>,
    /// <p>The name of the feature being evaluated.</p>
    #[doc(hidden)]
    pub feature: ::std::option::Option<::std::string::String>,
    /// <p>The name of the variation that was served to the user session.</p>
    #[doc(hidden)]
    pub variation: ::std::option::Option<::std::string::String>,
    /// <p>The value assigned to this variation to differentiate it from the other variations of this feature.</p>
    #[doc(hidden)]
    pub value: ::std::option::Option<crate::types::VariableValue>,
    /// <p>An internal ID that represents a unique user session of the application.</p>
    #[doc(hidden)]
    pub entity_id: ::std::option::Option<::std::string::String>,
    /// <p>Specifies the reason that the user session was assigned this variation. Possible values include <code>DEFAULT</code>, meaning the user was served the default variation; <code>LAUNCH_RULE_MATCH</code>, if the user session was enrolled in a launch; or <code>EXPERIMENT_RULE_MATCH</code>, if the user session was enrolled in an experiment.</p>
    #[doc(hidden)]
    pub reason: ::std::option::Option<::std::string::String>,
    /// <p>If this user was assigned to a launch or experiment, this field lists the launch or experiment name.</p>
    #[doc(hidden)]
    pub details: ::std::option::Option<::std::string::String>,
}
impl EvaluationResult {
    /// <p>The name or ARN of the project that contains the feature being evaluated.</p>
    pub fn project(&self) -> ::std::option::Option<&str> {
        self.project.as_deref()
    }
    /// <p>The name of the feature being evaluated.</p>
    pub fn feature(&self) -> ::std::option::Option<&str> {
        self.feature.as_deref()
    }
    /// <p>The name of the variation that was served to the user session.</p>
    pub fn variation(&self) -> ::std::option::Option<&str> {
        self.variation.as_deref()
    }
    /// <p>The value assigned to this variation to differentiate it from the other variations of this feature.</p>
    pub fn value(&self) -> ::std::option::Option<&crate::types::VariableValue> {
        self.value.as_ref()
    }
    /// <p>An internal ID that represents a unique user session of the application.</p>
    pub fn entity_id(&self) -> ::std::option::Option<&str> {
        self.entity_id.as_deref()
    }
    /// <p>Specifies the reason that the user session was assigned this variation. Possible values include <code>DEFAULT</code>, meaning the user was served the default variation; <code>LAUNCH_RULE_MATCH</code>, if the user session was enrolled in a launch; or <code>EXPERIMENT_RULE_MATCH</code>, if the user session was enrolled in an experiment.</p>
    pub fn reason(&self) -> ::std::option::Option<&str> {
        self.reason.as_deref()
    }
    /// <p>If this user was assigned to a launch or experiment, this field lists the launch or experiment name.</p>
    pub fn details(&self) -> ::std::option::Option<&str> {
        self.details.as_deref()
    }
}
impl EvaluationResult {
    /// Creates a new builder-style object to manufacture [`EvaluationResult`](crate::types::EvaluationResult).
    pub fn builder() -> crate::types::builders::EvaluationResultBuilder {
        crate::types::builders::EvaluationResultBuilder::default()
    }
}

/// A builder for [`EvaluationResult`](crate::types::EvaluationResult).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct EvaluationResultBuilder {
    pub(crate) project: ::std::option::Option<::std::string::String>,
    pub(crate) feature: ::std::option::Option<::std::string::String>,
    pub(crate) variation: ::std::option::Option<::std::string::String>,
    pub(crate) value: ::std::option::Option<crate::types::VariableValue>,
    pub(crate) entity_id: ::std::option::Option<::std::string::String>,
    pub(crate) reason: ::std::option::Option<::std::string::String>,
    pub(crate) details: ::std::option::Option<::std::string::String>,
}
impl EvaluationResultBuilder {
    /// <p>The name or ARN of the project that contains the feature being evaluated.</p>
    pub fn project(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.project = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name or ARN of the project that contains the feature being evaluated.</p>
    pub fn set_project(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.project = input;
        self
    }
    /// <p>The name of the feature being evaluated.</p>
    pub fn feature(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.feature = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the feature being evaluated.</p>
    pub fn set_feature(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.feature = input;
        self
    }
    /// <p>The name of the variation that was served to the user session.</p>
    pub fn variation(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.variation = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the variation that was served to the user session.</p>
    pub fn set_variation(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.variation = input;
        self
    }
    /// <p>The value assigned to this variation to differentiate it from the other variations of this feature.</p>
    pub fn value(mut self, input: crate::types::VariableValue) -> Self {
        self.value = ::std::option::Option::Some(input);
        self
    }
    /// <p>The value assigned to this variation to differentiate it from the other variations of this feature.</p>
    pub fn set_value(mut self, input: ::std::option::Option<crate::types::VariableValue>) -> Self {
        self.value = input;
        self
    }
    /// <p>An internal ID that represents a unique user session of the application.</p>
    pub fn entity_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.entity_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An internal ID that represents a unique user session of the application.</p>
    pub fn set_entity_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.entity_id = input;
        self
    }
    /// <p>Specifies the reason that the user session was assigned this variation. Possible values include <code>DEFAULT</code>, meaning the user was served the default variation; <code>LAUNCH_RULE_MATCH</code>, if the user session was enrolled in a launch; or <code>EXPERIMENT_RULE_MATCH</code>, if the user session was enrolled in an experiment.</p>
    pub fn reason(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.reason = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specifies the reason that the user session was assigned this variation. Possible values include <code>DEFAULT</code>, meaning the user was served the default variation; <code>LAUNCH_RULE_MATCH</code>, if the user session was enrolled in a launch; or <code>EXPERIMENT_RULE_MATCH</code>, if the user session was enrolled in an experiment.</p>
    pub fn set_reason(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.reason = input;
        self
    }
    /// <p>If this user was assigned to a launch or experiment, this field lists the launch or experiment name.</p>
    pub fn details(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.details = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If this user was assigned to a launch or experiment, this field lists the launch or experiment name.</p>
    pub fn set_details(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.details = input;
        self
    }
    /// Consumes the builder and constructs a [`EvaluationResult`](crate::types::EvaluationResult).
    pub fn build(self) -> crate::types::EvaluationResult {
        crate::types::EvaluationResult {
            project: self.project,
            feature: self.feature,
            variation: self.variation,
            value: self.value,
            entity_id: self.entity_id,
            reason: self.reason,
            details: self.details,
        }
    }
}
