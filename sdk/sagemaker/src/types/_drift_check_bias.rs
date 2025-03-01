// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the drift check bias baselines that can be used when the model monitor is set using the model package.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DriftCheckBias {
    /// <p>The bias config file for a model.</p>
    #[doc(hidden)]
    pub config_file: ::std::option::Option<crate::types::FileSource>,
    /// <p>The pre-training constraints.</p>
    #[doc(hidden)]
    pub pre_training_constraints: ::std::option::Option<crate::types::MetricsSource>,
    /// <p>The post-training constraints.</p>
    #[doc(hidden)]
    pub post_training_constraints: ::std::option::Option<crate::types::MetricsSource>,
}
impl DriftCheckBias {
    /// <p>The bias config file for a model.</p>
    pub fn config_file(&self) -> ::std::option::Option<&crate::types::FileSource> {
        self.config_file.as_ref()
    }
    /// <p>The pre-training constraints.</p>
    pub fn pre_training_constraints(&self) -> ::std::option::Option<&crate::types::MetricsSource> {
        self.pre_training_constraints.as_ref()
    }
    /// <p>The post-training constraints.</p>
    pub fn post_training_constraints(&self) -> ::std::option::Option<&crate::types::MetricsSource> {
        self.post_training_constraints.as_ref()
    }
}
impl DriftCheckBias {
    /// Creates a new builder-style object to manufacture [`DriftCheckBias`](crate::types::DriftCheckBias).
    pub fn builder() -> crate::types::builders::DriftCheckBiasBuilder {
        crate::types::builders::DriftCheckBiasBuilder::default()
    }
}

/// A builder for [`DriftCheckBias`](crate::types::DriftCheckBias).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DriftCheckBiasBuilder {
    pub(crate) config_file: ::std::option::Option<crate::types::FileSource>,
    pub(crate) pre_training_constraints: ::std::option::Option<crate::types::MetricsSource>,
    pub(crate) post_training_constraints: ::std::option::Option<crate::types::MetricsSource>,
}
impl DriftCheckBiasBuilder {
    /// <p>The bias config file for a model.</p>
    pub fn config_file(mut self, input: crate::types::FileSource) -> Self {
        self.config_file = ::std::option::Option::Some(input);
        self
    }
    /// <p>The bias config file for a model.</p>
    pub fn set_config_file(
        mut self,
        input: ::std::option::Option<crate::types::FileSource>,
    ) -> Self {
        self.config_file = input;
        self
    }
    /// <p>The pre-training constraints.</p>
    pub fn pre_training_constraints(mut self, input: crate::types::MetricsSource) -> Self {
        self.pre_training_constraints = ::std::option::Option::Some(input);
        self
    }
    /// <p>The pre-training constraints.</p>
    pub fn set_pre_training_constraints(
        mut self,
        input: ::std::option::Option<crate::types::MetricsSource>,
    ) -> Self {
        self.pre_training_constraints = input;
        self
    }
    /// <p>The post-training constraints.</p>
    pub fn post_training_constraints(mut self, input: crate::types::MetricsSource) -> Self {
        self.post_training_constraints = ::std::option::Option::Some(input);
        self
    }
    /// <p>The post-training constraints.</p>
    pub fn set_post_training_constraints(
        mut self,
        input: ::std::option::Option<crate::types::MetricsSource>,
    ) -> Self {
        self.post_training_constraints = input;
        self
    }
    /// Consumes the builder and constructs a [`DriftCheckBias`](crate::types::DriftCheckBias).
    pub fn build(self) -> crate::types::DriftCheckBias {
        crate::types::DriftCheckBias {
            config_file: self.config_file,
            pre_training_constraints: self.pre_training_constraints,
            post_training_constraints: self.post_training_constraints,
        }
    }
}
