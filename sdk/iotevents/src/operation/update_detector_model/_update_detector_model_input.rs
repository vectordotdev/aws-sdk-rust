// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateDetectorModelInput {
    /// <p>The name of the detector model that is updated.</p>
    #[doc(hidden)]
    pub detector_model_name: ::std::option::Option<::std::string::String>,
    /// <p>Information that defines how a detector operates.</p>
    #[doc(hidden)]
    pub detector_model_definition: ::std::option::Option<crate::types::DetectorModelDefinition>,
    /// <p>A brief description of the detector model.</p>
    #[doc(hidden)]
    pub detector_model_description: ::std::option::Option<::std::string::String>,
    /// <p>The ARN of the role that grants permission to AWS IoT Events to perform its operations.</p>
    #[doc(hidden)]
    pub role_arn: ::std::option::Option<::std::string::String>,
    /// <p>Information about the order in which events are evaluated and how actions are executed. </p>
    #[doc(hidden)]
    pub evaluation_method: ::std::option::Option<crate::types::EvaluationMethod>,
}
impl UpdateDetectorModelInput {
    /// <p>The name of the detector model that is updated.</p>
    pub fn detector_model_name(&self) -> ::std::option::Option<&str> {
        self.detector_model_name.as_deref()
    }
    /// <p>Information that defines how a detector operates.</p>
    pub fn detector_model_definition(
        &self,
    ) -> ::std::option::Option<&crate::types::DetectorModelDefinition> {
        self.detector_model_definition.as_ref()
    }
    /// <p>A brief description of the detector model.</p>
    pub fn detector_model_description(&self) -> ::std::option::Option<&str> {
        self.detector_model_description.as_deref()
    }
    /// <p>The ARN of the role that grants permission to AWS IoT Events to perform its operations.</p>
    pub fn role_arn(&self) -> ::std::option::Option<&str> {
        self.role_arn.as_deref()
    }
    /// <p>Information about the order in which events are evaluated and how actions are executed. </p>
    pub fn evaluation_method(&self) -> ::std::option::Option<&crate::types::EvaluationMethod> {
        self.evaluation_method.as_ref()
    }
}
impl UpdateDetectorModelInput {
    /// Creates a new builder-style object to manufacture [`UpdateDetectorModelInput`](crate::operation::update_detector_model::UpdateDetectorModelInput).
    pub fn builder(
    ) -> crate::operation::update_detector_model::builders::UpdateDetectorModelInputBuilder {
        crate::operation::update_detector_model::builders::UpdateDetectorModelInputBuilder::default(
        )
    }
}

/// A builder for [`UpdateDetectorModelInput`](crate::operation::update_detector_model::UpdateDetectorModelInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateDetectorModelInputBuilder {
    pub(crate) detector_model_name: ::std::option::Option<::std::string::String>,
    pub(crate) detector_model_definition:
        ::std::option::Option<crate::types::DetectorModelDefinition>,
    pub(crate) detector_model_description: ::std::option::Option<::std::string::String>,
    pub(crate) role_arn: ::std::option::Option<::std::string::String>,
    pub(crate) evaluation_method: ::std::option::Option<crate::types::EvaluationMethod>,
}
impl UpdateDetectorModelInputBuilder {
    /// <p>The name of the detector model that is updated.</p>
    pub fn detector_model_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.detector_model_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the detector model that is updated.</p>
    pub fn set_detector_model_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.detector_model_name = input;
        self
    }
    /// <p>Information that defines how a detector operates.</p>
    pub fn detector_model_definition(
        mut self,
        input: crate::types::DetectorModelDefinition,
    ) -> Self {
        self.detector_model_definition = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information that defines how a detector operates.</p>
    pub fn set_detector_model_definition(
        mut self,
        input: ::std::option::Option<crate::types::DetectorModelDefinition>,
    ) -> Self {
        self.detector_model_definition = input;
        self
    }
    /// <p>A brief description of the detector model.</p>
    pub fn detector_model_description(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.detector_model_description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A brief description of the detector model.</p>
    pub fn set_detector_model_description(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.detector_model_description = input;
        self
    }
    /// <p>The ARN of the role that grants permission to AWS IoT Events to perform its operations.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.role_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the role that grants permission to AWS IoT Events to perform its operations.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.role_arn = input;
        self
    }
    /// <p>Information about the order in which events are evaluated and how actions are executed. </p>
    pub fn evaluation_method(mut self, input: crate::types::EvaluationMethod) -> Self {
        self.evaluation_method = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the order in which events are evaluated and how actions are executed. </p>
    pub fn set_evaluation_method(
        mut self,
        input: ::std::option::Option<crate::types::EvaluationMethod>,
    ) -> Self {
        self.evaluation_method = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateDetectorModelInput`](crate::operation::update_detector_model::UpdateDetectorModelInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_detector_model::UpdateDetectorModelInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::update_detector_model::UpdateDetectorModelInput {
                detector_model_name: self.detector_model_name,
                detector_model_definition: self.detector_model_definition,
                detector_model_description: self.detector_model_description,
                role_arn: self.role_arn,
                evaluation_method: self.evaluation_method,
            },
        )
    }
}
