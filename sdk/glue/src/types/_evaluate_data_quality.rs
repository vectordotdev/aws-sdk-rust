// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies your data quality evaluation criteria.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EvaluateDataQuality {
    /// <p>The name of the data quality evaluation.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The inputs of your data quality evaluation.</p>
    #[doc(hidden)]
    pub inputs: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The ruleset for your data quality evaluation.</p>
    #[doc(hidden)]
    pub ruleset: ::std::option::Option<::std::string::String>,
    /// <p>The output of your data quality evaluation.</p>
    #[doc(hidden)]
    pub output: ::std::option::Option<crate::types::DqTransformOutput>,
    /// <p>Options to configure how your results are published.</p>
    #[doc(hidden)]
    pub publishing_options: ::std::option::Option<crate::types::DqResultsPublishingOptions>,
    /// <p>Options to configure how your job will stop if your data quality evaluation fails.</p>
    #[doc(hidden)]
    pub stop_job_on_failure_options: ::std::option::Option<crate::types::DqStopJobOnFailureOptions>,
}
impl EvaluateDataQuality {
    /// <p>The name of the data quality evaluation.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The inputs of your data quality evaluation.</p>
    pub fn inputs(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.inputs.as_deref()
    }
    /// <p>The ruleset for your data quality evaluation.</p>
    pub fn ruleset(&self) -> ::std::option::Option<&str> {
        self.ruleset.as_deref()
    }
    /// <p>The output of your data quality evaluation.</p>
    pub fn output(&self) -> ::std::option::Option<&crate::types::DqTransformOutput> {
        self.output.as_ref()
    }
    /// <p>Options to configure how your results are published.</p>
    pub fn publishing_options(
        &self,
    ) -> ::std::option::Option<&crate::types::DqResultsPublishingOptions> {
        self.publishing_options.as_ref()
    }
    /// <p>Options to configure how your job will stop if your data quality evaluation fails.</p>
    pub fn stop_job_on_failure_options(
        &self,
    ) -> ::std::option::Option<&crate::types::DqStopJobOnFailureOptions> {
        self.stop_job_on_failure_options.as_ref()
    }
}
impl EvaluateDataQuality {
    /// Creates a new builder-style object to manufacture [`EvaluateDataQuality`](crate::types::EvaluateDataQuality).
    pub fn builder() -> crate::types::builders::EvaluateDataQualityBuilder {
        crate::types::builders::EvaluateDataQualityBuilder::default()
    }
}

/// A builder for [`EvaluateDataQuality`](crate::types::EvaluateDataQuality).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct EvaluateDataQualityBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) inputs: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) ruleset: ::std::option::Option<::std::string::String>,
    pub(crate) output: ::std::option::Option<crate::types::DqTransformOutput>,
    pub(crate) publishing_options: ::std::option::Option<crate::types::DqResultsPublishingOptions>,
    pub(crate) stop_job_on_failure_options:
        ::std::option::Option<crate::types::DqStopJobOnFailureOptions>,
}
impl EvaluateDataQualityBuilder {
    /// <p>The name of the data quality evaluation.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the data quality evaluation.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Appends an item to `inputs`.
    ///
    /// To override the contents of this collection use [`set_inputs`](Self::set_inputs).
    ///
    /// <p>The inputs of your data quality evaluation.</p>
    pub fn inputs(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.inputs.unwrap_or_default();
        v.push(input.into());
        self.inputs = ::std::option::Option::Some(v);
        self
    }
    /// <p>The inputs of your data quality evaluation.</p>
    pub fn set_inputs(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inputs = input;
        self
    }
    /// <p>The ruleset for your data quality evaluation.</p>
    pub fn ruleset(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.ruleset = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ruleset for your data quality evaluation.</p>
    pub fn set_ruleset(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.ruleset = input;
        self
    }
    /// <p>The output of your data quality evaluation.</p>
    pub fn output(mut self, input: crate::types::DqTransformOutput) -> Self {
        self.output = ::std::option::Option::Some(input);
        self
    }
    /// <p>The output of your data quality evaluation.</p>
    pub fn set_output(
        mut self,
        input: ::std::option::Option<crate::types::DqTransformOutput>,
    ) -> Self {
        self.output = input;
        self
    }
    /// <p>Options to configure how your results are published.</p>
    pub fn publishing_options(mut self, input: crate::types::DqResultsPublishingOptions) -> Self {
        self.publishing_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>Options to configure how your results are published.</p>
    pub fn set_publishing_options(
        mut self,
        input: ::std::option::Option<crate::types::DqResultsPublishingOptions>,
    ) -> Self {
        self.publishing_options = input;
        self
    }
    /// <p>Options to configure how your job will stop if your data quality evaluation fails.</p>
    pub fn stop_job_on_failure_options(
        mut self,
        input: crate::types::DqStopJobOnFailureOptions,
    ) -> Self {
        self.stop_job_on_failure_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>Options to configure how your job will stop if your data quality evaluation fails.</p>
    pub fn set_stop_job_on_failure_options(
        mut self,
        input: ::std::option::Option<crate::types::DqStopJobOnFailureOptions>,
    ) -> Self {
        self.stop_job_on_failure_options = input;
        self
    }
    /// Consumes the builder and constructs a [`EvaluateDataQuality`](crate::types::EvaluateDataQuality).
    pub fn build(self) -> crate::types::EvaluateDataQuality {
        crate::types::EvaluateDataQuality {
            name: self.name,
            inputs: self.inputs,
            ruleset: self.ruleset,
            output: self.output,
            publishing_options: self.publishing_options,
            stop_job_on_failure_options: self.stop_job_on_failure_options,
        }
    }
}
