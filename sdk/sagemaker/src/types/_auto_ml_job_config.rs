// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A collection of settings used for an AutoML job.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AutoMlJobConfig {
    /// <p>How long an AutoML job is allowed to run, or how many candidates a job is allowed to generate.</p>
    #[doc(hidden)]
    pub completion_criteria: ::std::option::Option<crate::types::AutoMlJobCompletionCriteria>,
    /// <p>The security configuration for traffic encryption or Amazon VPC settings.</p>
    #[doc(hidden)]
    pub security_config: ::std::option::Option<crate::types::AutoMlSecurityConfig>,
    /// <p>The configuration for splitting the input training dataset.</p>
    /// <p>Type: AutoMLDataSplitConfig</p>
    #[doc(hidden)]
    pub data_split_config: ::std::option::Option<crate::types::AutoMlDataSplitConfig>,
    /// <p>The configuration for generating a candidate for an AutoML job (optional). </p>
    #[doc(hidden)]
    pub candidate_generation_config:
        ::std::option::Option<crate::types::AutoMlCandidateGenerationConfig>,
    /// <p>The method that Autopilot uses to train the data. You can either specify the mode manually or let Autopilot choose for you based on the dataset size by selecting <code>AUTO</code>. In <code>AUTO</code> mode, Autopilot chooses <code>ENSEMBLING</code> for datasets smaller than 100 MB, and <code>HYPERPARAMETER_TUNING</code> for larger ones.</p>
    /// <p>The <code>ENSEMBLING</code> mode uses a multi-stack ensemble model to predict classification and regression tasks directly from your dataset. This machine learning mode combines several base models to produce an optimal predictive model. It then uses a stacking ensemble method to combine predictions from contributing members. A multi-stack ensemble model can provide better performance over a single model by combining the predictive capabilities of multiple models. See <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/autopilot-model-support-validation.html#autopilot-algorithm-suppprt">Autopilot algorithm support</a> for a list of algorithms supported by <code>ENSEMBLING</code> mode.</p>
    /// <p>The <code>HYPERPARAMETER_TUNING</code> (HPO) mode uses the best hyperparameters to train the best version of a model. HPO automatically selects an algorithm for the type of problem you want to solve. Then HPO finds the best hyperparameters according to your objective metric. See <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/autopilot-model-support-validation.html#autopilot-algorithm-suppprt">Autopilot algorithm support</a> for a list of algorithms supported by <code>HYPERPARAMETER_TUNING</code> mode.</p>
    #[doc(hidden)]
    pub mode: ::std::option::Option<crate::types::AutoMlMode>,
}
impl AutoMlJobConfig {
    /// <p>How long an AutoML job is allowed to run, or how many candidates a job is allowed to generate.</p>
    pub fn completion_criteria(
        &self,
    ) -> ::std::option::Option<&crate::types::AutoMlJobCompletionCriteria> {
        self.completion_criteria.as_ref()
    }
    /// <p>The security configuration for traffic encryption or Amazon VPC settings.</p>
    pub fn security_config(&self) -> ::std::option::Option<&crate::types::AutoMlSecurityConfig> {
        self.security_config.as_ref()
    }
    /// <p>The configuration for splitting the input training dataset.</p>
    /// <p>Type: AutoMLDataSplitConfig</p>
    pub fn data_split_config(&self) -> ::std::option::Option<&crate::types::AutoMlDataSplitConfig> {
        self.data_split_config.as_ref()
    }
    /// <p>The configuration for generating a candidate for an AutoML job (optional). </p>
    pub fn candidate_generation_config(
        &self,
    ) -> ::std::option::Option<&crate::types::AutoMlCandidateGenerationConfig> {
        self.candidate_generation_config.as_ref()
    }
    /// <p>The method that Autopilot uses to train the data. You can either specify the mode manually or let Autopilot choose for you based on the dataset size by selecting <code>AUTO</code>. In <code>AUTO</code> mode, Autopilot chooses <code>ENSEMBLING</code> for datasets smaller than 100 MB, and <code>HYPERPARAMETER_TUNING</code> for larger ones.</p>
    /// <p>The <code>ENSEMBLING</code> mode uses a multi-stack ensemble model to predict classification and regression tasks directly from your dataset. This machine learning mode combines several base models to produce an optimal predictive model. It then uses a stacking ensemble method to combine predictions from contributing members. A multi-stack ensemble model can provide better performance over a single model by combining the predictive capabilities of multiple models. See <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/autopilot-model-support-validation.html#autopilot-algorithm-suppprt">Autopilot algorithm support</a> for a list of algorithms supported by <code>ENSEMBLING</code> mode.</p>
    /// <p>The <code>HYPERPARAMETER_TUNING</code> (HPO) mode uses the best hyperparameters to train the best version of a model. HPO automatically selects an algorithm for the type of problem you want to solve. Then HPO finds the best hyperparameters according to your objective metric. See <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/autopilot-model-support-validation.html#autopilot-algorithm-suppprt">Autopilot algorithm support</a> for a list of algorithms supported by <code>HYPERPARAMETER_TUNING</code> mode.</p>
    pub fn mode(&self) -> ::std::option::Option<&crate::types::AutoMlMode> {
        self.mode.as_ref()
    }
}
impl AutoMlJobConfig {
    /// Creates a new builder-style object to manufacture [`AutoMlJobConfig`](crate::types::AutoMlJobConfig).
    pub fn builder() -> crate::types::builders::AutoMlJobConfigBuilder {
        crate::types::builders::AutoMlJobConfigBuilder::default()
    }
}

/// A builder for [`AutoMlJobConfig`](crate::types::AutoMlJobConfig).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AutoMlJobConfigBuilder {
    pub(crate) completion_criteria:
        ::std::option::Option<crate::types::AutoMlJobCompletionCriteria>,
    pub(crate) security_config: ::std::option::Option<crate::types::AutoMlSecurityConfig>,
    pub(crate) data_split_config: ::std::option::Option<crate::types::AutoMlDataSplitConfig>,
    pub(crate) candidate_generation_config:
        ::std::option::Option<crate::types::AutoMlCandidateGenerationConfig>,
    pub(crate) mode: ::std::option::Option<crate::types::AutoMlMode>,
}
impl AutoMlJobConfigBuilder {
    /// <p>How long an AutoML job is allowed to run, or how many candidates a job is allowed to generate.</p>
    pub fn completion_criteria(mut self, input: crate::types::AutoMlJobCompletionCriteria) -> Self {
        self.completion_criteria = ::std::option::Option::Some(input);
        self
    }
    /// <p>How long an AutoML job is allowed to run, or how many candidates a job is allowed to generate.</p>
    pub fn set_completion_criteria(
        mut self,
        input: ::std::option::Option<crate::types::AutoMlJobCompletionCriteria>,
    ) -> Self {
        self.completion_criteria = input;
        self
    }
    /// <p>The security configuration for traffic encryption or Amazon VPC settings.</p>
    pub fn security_config(mut self, input: crate::types::AutoMlSecurityConfig) -> Self {
        self.security_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>The security configuration for traffic encryption or Amazon VPC settings.</p>
    pub fn set_security_config(
        mut self,
        input: ::std::option::Option<crate::types::AutoMlSecurityConfig>,
    ) -> Self {
        self.security_config = input;
        self
    }
    /// <p>The configuration for splitting the input training dataset.</p>
    /// <p>Type: AutoMLDataSplitConfig</p>
    pub fn data_split_config(mut self, input: crate::types::AutoMlDataSplitConfig) -> Self {
        self.data_split_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>The configuration for splitting the input training dataset.</p>
    /// <p>Type: AutoMLDataSplitConfig</p>
    pub fn set_data_split_config(
        mut self,
        input: ::std::option::Option<crate::types::AutoMlDataSplitConfig>,
    ) -> Self {
        self.data_split_config = input;
        self
    }
    /// <p>The configuration for generating a candidate for an AutoML job (optional). </p>
    pub fn candidate_generation_config(
        mut self,
        input: crate::types::AutoMlCandidateGenerationConfig,
    ) -> Self {
        self.candidate_generation_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>The configuration for generating a candidate for an AutoML job (optional). </p>
    pub fn set_candidate_generation_config(
        mut self,
        input: ::std::option::Option<crate::types::AutoMlCandidateGenerationConfig>,
    ) -> Self {
        self.candidate_generation_config = input;
        self
    }
    /// <p>The method that Autopilot uses to train the data. You can either specify the mode manually or let Autopilot choose for you based on the dataset size by selecting <code>AUTO</code>. In <code>AUTO</code> mode, Autopilot chooses <code>ENSEMBLING</code> for datasets smaller than 100 MB, and <code>HYPERPARAMETER_TUNING</code> for larger ones.</p>
    /// <p>The <code>ENSEMBLING</code> mode uses a multi-stack ensemble model to predict classification and regression tasks directly from your dataset. This machine learning mode combines several base models to produce an optimal predictive model. It then uses a stacking ensemble method to combine predictions from contributing members. A multi-stack ensemble model can provide better performance over a single model by combining the predictive capabilities of multiple models. See <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/autopilot-model-support-validation.html#autopilot-algorithm-suppprt">Autopilot algorithm support</a> for a list of algorithms supported by <code>ENSEMBLING</code> mode.</p>
    /// <p>The <code>HYPERPARAMETER_TUNING</code> (HPO) mode uses the best hyperparameters to train the best version of a model. HPO automatically selects an algorithm for the type of problem you want to solve. Then HPO finds the best hyperparameters according to your objective metric. See <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/autopilot-model-support-validation.html#autopilot-algorithm-suppprt">Autopilot algorithm support</a> for a list of algorithms supported by <code>HYPERPARAMETER_TUNING</code> mode.</p>
    pub fn mode(mut self, input: crate::types::AutoMlMode) -> Self {
        self.mode = ::std::option::Option::Some(input);
        self
    }
    /// <p>The method that Autopilot uses to train the data. You can either specify the mode manually or let Autopilot choose for you based on the dataset size by selecting <code>AUTO</code>. In <code>AUTO</code> mode, Autopilot chooses <code>ENSEMBLING</code> for datasets smaller than 100 MB, and <code>HYPERPARAMETER_TUNING</code> for larger ones.</p>
    /// <p>The <code>ENSEMBLING</code> mode uses a multi-stack ensemble model to predict classification and regression tasks directly from your dataset. This machine learning mode combines several base models to produce an optimal predictive model. It then uses a stacking ensemble method to combine predictions from contributing members. A multi-stack ensemble model can provide better performance over a single model by combining the predictive capabilities of multiple models. See <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/autopilot-model-support-validation.html#autopilot-algorithm-suppprt">Autopilot algorithm support</a> for a list of algorithms supported by <code>ENSEMBLING</code> mode.</p>
    /// <p>The <code>HYPERPARAMETER_TUNING</code> (HPO) mode uses the best hyperparameters to train the best version of a model. HPO automatically selects an algorithm for the type of problem you want to solve. Then HPO finds the best hyperparameters according to your objective metric. See <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/autopilot-model-support-validation.html#autopilot-algorithm-suppprt">Autopilot algorithm support</a> for a list of algorithms supported by <code>HYPERPARAMETER_TUNING</code> mode.</p>
    pub fn set_mode(mut self, input: ::std::option::Option<crate::types::AutoMlMode>) -> Self {
        self.mode = input;
        self
    }
    /// Consumes the builder and constructs a [`AutoMlJobConfig`](crate::types::AutoMlJobConfig).
    pub fn build(self) -> crate::types::AutoMlJobConfig {
        crate::types::AutoMlJobConfig {
            completion_criteria: self.completion_criteria,
            security_config: self.security_config,
            data_split_config: self.data_split_config,
            candidate_generation_config: self.candidate_generation_config,
            mode: self.mode,
        }
    }
}
