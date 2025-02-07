// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the drift check explainability baselines that can be used when the model monitor is set using the model package. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DriftCheckExplainability {
    /// <p>The drift check explainability constraints.</p>
    #[doc(hidden)]
    pub constraints: ::std::option::Option<crate::types::MetricsSource>,
    /// <p>The explainability config file for the model.</p>
    #[doc(hidden)]
    pub config_file: ::std::option::Option<crate::types::FileSource>,
}
impl DriftCheckExplainability {
    /// <p>The drift check explainability constraints.</p>
    pub fn constraints(&self) -> ::std::option::Option<&crate::types::MetricsSource> {
        self.constraints.as_ref()
    }
    /// <p>The explainability config file for the model.</p>
    pub fn config_file(&self) -> ::std::option::Option<&crate::types::FileSource> {
        self.config_file.as_ref()
    }
}
impl DriftCheckExplainability {
    /// Creates a new builder-style object to manufacture [`DriftCheckExplainability`](crate::types::DriftCheckExplainability).
    pub fn builder() -> crate::types::builders::DriftCheckExplainabilityBuilder {
        crate::types::builders::DriftCheckExplainabilityBuilder::default()
    }
}

/// A builder for [`DriftCheckExplainability`](crate::types::DriftCheckExplainability).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DriftCheckExplainabilityBuilder {
    pub(crate) constraints: ::std::option::Option<crate::types::MetricsSource>,
    pub(crate) config_file: ::std::option::Option<crate::types::FileSource>,
}
impl DriftCheckExplainabilityBuilder {
    /// <p>The drift check explainability constraints.</p>
    pub fn constraints(mut self, input: crate::types::MetricsSource) -> Self {
        self.constraints = ::std::option::Option::Some(input);
        self
    }
    /// <p>The drift check explainability constraints.</p>
    pub fn set_constraints(
        mut self,
        input: ::std::option::Option<crate::types::MetricsSource>,
    ) -> Self {
        self.constraints = input;
        self
    }
    /// <p>The explainability config file for the model.</p>
    pub fn config_file(mut self, input: crate::types::FileSource) -> Self {
        self.config_file = ::std::option::Option::Some(input);
        self
    }
    /// <p>The explainability config file for the model.</p>
    pub fn set_config_file(
        mut self,
        input: ::std::option::Option<crate::types::FileSource>,
    ) -> Self {
        self.config_file = input;
        self
    }
    /// Consumes the builder and constructs a [`DriftCheckExplainability`](crate::types::DriftCheckExplainability).
    pub fn build(self) -> crate::types::DriftCheckExplainability {
        crate::types::DriftCheckExplainability {
            constraints: self.constraints,
            config_file: self.config_file,
        }
    }
}
