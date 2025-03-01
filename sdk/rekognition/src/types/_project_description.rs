// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A description of an Amazon Rekognition Custom Labels project. For more information, see <code>DescribeProjects</code>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ProjectDescription {
    /// <p>The Amazon Resource Name (ARN) of the project.</p>
    #[doc(hidden)]
    pub project_arn: ::std::option::Option<::std::string::String>,
    /// <p>The Unix timestamp for the date and time that the project was created.</p>
    #[doc(hidden)]
    pub creation_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The current status of the project.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::ProjectStatus>,
    /// <p> Information about the training and test datasets in the project. </p>
    #[doc(hidden)]
    pub datasets: ::std::option::Option<::std::vec::Vec<crate::types::DatasetMetadata>>,
}
impl ProjectDescription {
    /// <p>The Amazon Resource Name (ARN) of the project.</p>
    pub fn project_arn(&self) -> ::std::option::Option<&str> {
        self.project_arn.as_deref()
    }
    /// <p>The Unix timestamp for the date and time that the project was created.</p>
    pub fn creation_timestamp(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.creation_timestamp.as_ref()
    }
    /// <p>The current status of the project.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::ProjectStatus> {
        self.status.as_ref()
    }
    /// <p> Information about the training and test datasets in the project. </p>
    pub fn datasets(&self) -> ::std::option::Option<&[crate::types::DatasetMetadata]> {
        self.datasets.as_deref()
    }
}
impl ProjectDescription {
    /// Creates a new builder-style object to manufacture [`ProjectDescription`](crate::types::ProjectDescription).
    pub fn builder() -> crate::types::builders::ProjectDescriptionBuilder {
        crate::types::builders::ProjectDescriptionBuilder::default()
    }
}

/// A builder for [`ProjectDescription`](crate::types::ProjectDescription).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ProjectDescriptionBuilder {
    pub(crate) project_arn: ::std::option::Option<::std::string::String>,
    pub(crate) creation_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) status: ::std::option::Option<crate::types::ProjectStatus>,
    pub(crate) datasets: ::std::option::Option<::std::vec::Vec<crate::types::DatasetMetadata>>,
}
impl ProjectDescriptionBuilder {
    /// <p>The Amazon Resource Name (ARN) of the project.</p>
    pub fn project_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.project_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the project.</p>
    pub fn set_project_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.project_arn = input;
        self
    }
    /// <p>The Unix timestamp for the date and time that the project was created.</p>
    pub fn creation_timestamp(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.creation_timestamp = ::std::option::Option::Some(input);
        self
    }
    /// <p>The Unix timestamp for the date and time that the project was created.</p>
    pub fn set_creation_timestamp(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.creation_timestamp = input;
        self
    }
    /// <p>The current status of the project.</p>
    pub fn status(mut self, input: crate::types::ProjectStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The current status of the project.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::ProjectStatus>) -> Self {
        self.status = input;
        self
    }
    /// Appends an item to `datasets`.
    ///
    /// To override the contents of this collection use [`set_datasets`](Self::set_datasets).
    ///
    /// <p> Information about the training and test datasets in the project. </p>
    pub fn datasets(mut self, input: crate::types::DatasetMetadata) -> Self {
        let mut v = self.datasets.unwrap_or_default();
        v.push(input);
        self.datasets = ::std::option::Option::Some(v);
        self
    }
    /// <p> Information about the training and test datasets in the project. </p>
    pub fn set_datasets(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::DatasetMetadata>>,
    ) -> Self {
        self.datasets = input;
        self
    }
    /// Consumes the builder and constructs a [`ProjectDescription`](crate::types::ProjectDescription).
    pub fn build(self) -> crate::types::ProjectDescription {
        crate::types::ProjectDescription {
            project_arn: self.project_arn,
            creation_timestamp: self.creation_timestamp,
            status: self.status,
            datasets: self.datasets,
        }
    }
}
