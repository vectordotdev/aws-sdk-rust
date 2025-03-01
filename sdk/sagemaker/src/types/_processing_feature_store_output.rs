// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Configuration for processing job outputs in Amazon SageMaker Feature Store.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ProcessingFeatureStoreOutput {
    /// <p>The name of the Amazon SageMaker FeatureGroup to use as the destination for processing job output. Note that your processing script is responsible for putting records into your Feature Store.</p>
    #[doc(hidden)]
    pub feature_group_name: ::std::option::Option<::std::string::String>,
}
impl ProcessingFeatureStoreOutput {
    /// <p>The name of the Amazon SageMaker FeatureGroup to use as the destination for processing job output. Note that your processing script is responsible for putting records into your Feature Store.</p>
    pub fn feature_group_name(&self) -> ::std::option::Option<&str> {
        self.feature_group_name.as_deref()
    }
}
impl ProcessingFeatureStoreOutput {
    /// Creates a new builder-style object to manufacture [`ProcessingFeatureStoreOutput`](crate::types::ProcessingFeatureStoreOutput).
    pub fn builder() -> crate::types::builders::ProcessingFeatureStoreOutputBuilder {
        crate::types::builders::ProcessingFeatureStoreOutputBuilder::default()
    }
}

/// A builder for [`ProcessingFeatureStoreOutput`](crate::types::ProcessingFeatureStoreOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ProcessingFeatureStoreOutputBuilder {
    pub(crate) feature_group_name: ::std::option::Option<::std::string::String>,
}
impl ProcessingFeatureStoreOutputBuilder {
    /// <p>The name of the Amazon SageMaker FeatureGroup to use as the destination for processing job output. Note that your processing script is responsible for putting records into your Feature Store.</p>
    pub fn feature_group_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.feature_group_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the Amazon SageMaker FeatureGroup to use as the destination for processing job output. Note that your processing script is responsible for putting records into your Feature Store.</p>
    pub fn set_feature_group_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.feature_group_name = input;
        self
    }
    /// Consumes the builder and constructs a [`ProcessingFeatureStoreOutput`](crate::types::ProcessingFeatureStoreOutput).
    pub fn build(self) -> crate::types::ProcessingFeatureStoreOutput {
        crate::types::ProcessingFeatureStoreOutput {
            feature_group_name: self.feature_group_name,
        }
    }
}
