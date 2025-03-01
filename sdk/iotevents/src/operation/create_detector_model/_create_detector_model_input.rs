// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateDetectorModelInput {
    /// <p>The name of the detector model.</p>
    #[doc(hidden)]
    pub detector_model_name: ::std::option::Option<::std::string::String>,
    /// <p>Information that defines how the detectors operate.</p>
    #[doc(hidden)]
    pub detector_model_definition: ::std::option::Option<crate::types::DetectorModelDefinition>,
    /// <p>A brief description of the detector model.</p>
    #[doc(hidden)]
    pub detector_model_description: ::std::option::Option<::std::string::String>,
    /// <p>The input attribute key used to identify a device or system to create a detector (an instance of the detector model) and then to route each input received to the appropriate detector (instance). This parameter uses a JSON-path expression in the message payload of each input to specify the attribute-value pair that is used to identify the device associated with the input.</p>
    #[doc(hidden)]
    pub key: ::std::option::Option<::std::string::String>,
    /// <p>The ARN of the role that grants permission to AWS IoT Events to perform its operations.</p>
    #[doc(hidden)]
    pub role_arn: ::std::option::Option<::std::string::String>,
    /// <p>Metadata that can be used to manage the detector model.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    /// <p>Information about the order in which events are evaluated and how actions are executed. </p>
    #[doc(hidden)]
    pub evaluation_method: ::std::option::Option<crate::types::EvaluationMethod>,
}
impl CreateDetectorModelInput {
    /// <p>The name of the detector model.</p>
    pub fn detector_model_name(&self) -> ::std::option::Option<&str> {
        self.detector_model_name.as_deref()
    }
    /// <p>Information that defines how the detectors operate.</p>
    pub fn detector_model_definition(
        &self,
    ) -> ::std::option::Option<&crate::types::DetectorModelDefinition> {
        self.detector_model_definition.as_ref()
    }
    /// <p>A brief description of the detector model.</p>
    pub fn detector_model_description(&self) -> ::std::option::Option<&str> {
        self.detector_model_description.as_deref()
    }
    /// <p>The input attribute key used to identify a device or system to create a detector (an instance of the detector model) and then to route each input received to the appropriate detector (instance). This parameter uses a JSON-path expression in the message payload of each input to specify the attribute-value pair that is used to identify the device associated with the input.</p>
    pub fn key(&self) -> ::std::option::Option<&str> {
        self.key.as_deref()
    }
    /// <p>The ARN of the role that grants permission to AWS IoT Events to perform its operations.</p>
    pub fn role_arn(&self) -> ::std::option::Option<&str> {
        self.role_arn.as_deref()
    }
    /// <p>Metadata that can be used to manage the detector model.</p>
    pub fn tags(&self) -> ::std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
    /// <p>Information about the order in which events are evaluated and how actions are executed. </p>
    pub fn evaluation_method(&self) -> ::std::option::Option<&crate::types::EvaluationMethod> {
        self.evaluation_method.as_ref()
    }
}
impl CreateDetectorModelInput {
    /// Creates a new builder-style object to manufacture [`CreateDetectorModelInput`](crate::operation::create_detector_model::CreateDetectorModelInput).
    pub fn builder(
    ) -> crate::operation::create_detector_model::builders::CreateDetectorModelInputBuilder {
        crate::operation::create_detector_model::builders::CreateDetectorModelInputBuilder::default(
        )
    }
}

/// A builder for [`CreateDetectorModelInput`](crate::operation::create_detector_model::CreateDetectorModelInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateDetectorModelInputBuilder {
    pub(crate) detector_model_name: ::std::option::Option<::std::string::String>,
    pub(crate) detector_model_definition:
        ::std::option::Option<crate::types::DetectorModelDefinition>,
    pub(crate) detector_model_description: ::std::option::Option<::std::string::String>,
    pub(crate) key: ::std::option::Option<::std::string::String>,
    pub(crate) role_arn: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    pub(crate) evaluation_method: ::std::option::Option<crate::types::EvaluationMethod>,
}
impl CreateDetectorModelInputBuilder {
    /// <p>The name of the detector model.</p>
    pub fn detector_model_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.detector_model_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the detector model.</p>
    pub fn set_detector_model_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.detector_model_name = input;
        self
    }
    /// <p>Information that defines how the detectors operate.</p>
    pub fn detector_model_definition(
        mut self,
        input: crate::types::DetectorModelDefinition,
    ) -> Self {
        self.detector_model_definition = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information that defines how the detectors operate.</p>
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
    /// <p>The input attribute key used to identify a device or system to create a detector (an instance of the detector model) and then to route each input received to the appropriate detector (instance). This parameter uses a JSON-path expression in the message payload of each input to specify the attribute-value pair that is used to identify the device associated with the input.</p>
    pub fn key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The input attribute key used to identify a device or system to create a detector (an instance of the detector model) and then to route each input received to the appropriate detector (instance). This parameter uses a JSON-path expression in the message payload of each input to specify the attribute-value pair that is used to identify the device associated with the input.</p>
    pub fn set_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key = input;
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
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Metadata that can be used to manage the detector model.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>Metadata that can be used to manage the detector model.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
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
    /// Consumes the builder and constructs a [`CreateDetectorModelInput`](crate::operation::create_detector_model::CreateDetectorModelInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_detector_model::CreateDetectorModelInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::create_detector_model::CreateDetectorModelInput {
                detector_model_name: self.detector_model_name,
                detector_model_definition: self.detector_model_definition,
                detector_model_description: self.detector_model_description,
                key: self.key,
                role_arn: self.role_arn,
                tags: self.tags,
                evaluation_method: self.evaluation_method,
            },
        )
    }
}
