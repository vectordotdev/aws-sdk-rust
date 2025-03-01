// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A SageMaker image. A SageMaker image represents a set of container images that are derived from a common base container image. Each of these container images is represented by a SageMaker <code>ImageVersion</code>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Image {
    /// <p>When the image was created.</p>
    #[doc(hidden)]
    pub creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The description of the image.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The name of the image as displayed.</p>
    #[doc(hidden)]
    pub display_name: ::std::option::Option<::std::string::String>,
    /// <p>When a create, update, or delete operation fails, the reason for the failure.</p>
    #[doc(hidden)]
    pub failure_reason: ::std::option::Option<::std::string::String>,
    /// <p>The ARN of the image.</p>
    #[doc(hidden)]
    pub image_arn: ::std::option::Option<::std::string::String>,
    /// <p>The name of the image.</p>
    #[doc(hidden)]
    pub image_name: ::std::option::Option<::std::string::String>,
    /// <p>The status of the image.</p>
    #[doc(hidden)]
    pub image_status: ::std::option::Option<crate::types::ImageStatus>,
    /// <p>When the image was last modified.</p>
    #[doc(hidden)]
    pub last_modified_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl Image {
    /// <p>When the image was created.</p>
    pub fn creation_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.creation_time.as_ref()
    }
    /// <p>The description of the image.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The name of the image as displayed.</p>
    pub fn display_name(&self) -> ::std::option::Option<&str> {
        self.display_name.as_deref()
    }
    /// <p>When a create, update, or delete operation fails, the reason for the failure.</p>
    pub fn failure_reason(&self) -> ::std::option::Option<&str> {
        self.failure_reason.as_deref()
    }
    /// <p>The ARN of the image.</p>
    pub fn image_arn(&self) -> ::std::option::Option<&str> {
        self.image_arn.as_deref()
    }
    /// <p>The name of the image.</p>
    pub fn image_name(&self) -> ::std::option::Option<&str> {
        self.image_name.as_deref()
    }
    /// <p>The status of the image.</p>
    pub fn image_status(&self) -> ::std::option::Option<&crate::types::ImageStatus> {
        self.image_status.as_ref()
    }
    /// <p>When the image was last modified.</p>
    pub fn last_modified_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_modified_time.as_ref()
    }
}
impl Image {
    /// Creates a new builder-style object to manufacture [`Image`](crate::types::Image).
    pub fn builder() -> crate::types::builders::ImageBuilder {
        crate::types::builders::ImageBuilder::default()
    }
}

/// A builder for [`Image`](crate::types::Image).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ImageBuilder {
    pub(crate) creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) display_name: ::std::option::Option<::std::string::String>,
    pub(crate) failure_reason: ::std::option::Option<::std::string::String>,
    pub(crate) image_arn: ::std::option::Option<::std::string::String>,
    pub(crate) image_name: ::std::option::Option<::std::string::String>,
    pub(crate) image_status: ::std::option::Option<crate::types::ImageStatus>,
    pub(crate) last_modified_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl ImageBuilder {
    /// <p>When the image was created.</p>
    pub fn creation_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.creation_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>When the image was created.</p>
    pub fn set_creation_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.creation_time = input;
        self
    }
    /// <p>The description of the image.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description of the image.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The name of the image as displayed.</p>
    pub fn display_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.display_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the image as displayed.</p>
    pub fn set_display_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.display_name = input;
        self
    }
    /// <p>When a create, update, or delete operation fails, the reason for the failure.</p>
    pub fn failure_reason(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.failure_reason = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>When a create, update, or delete operation fails, the reason for the failure.</p>
    pub fn set_failure_reason(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.failure_reason = input;
        self
    }
    /// <p>The ARN of the image.</p>
    pub fn image_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.image_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the image.</p>
    pub fn set_image_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.image_arn = input;
        self
    }
    /// <p>The name of the image.</p>
    pub fn image_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.image_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the image.</p>
    pub fn set_image_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.image_name = input;
        self
    }
    /// <p>The status of the image.</p>
    pub fn image_status(mut self, input: crate::types::ImageStatus) -> Self {
        self.image_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the image.</p>
    pub fn set_image_status(
        mut self,
        input: ::std::option::Option<crate::types::ImageStatus>,
    ) -> Self {
        self.image_status = input;
        self
    }
    /// <p>When the image was last modified.</p>
    pub fn last_modified_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_modified_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>When the image was last modified.</p>
    pub fn set_last_modified_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.last_modified_time = input;
        self
    }
    /// Consumes the builder and constructs a [`Image`](crate::types::Image).
    pub fn build(self) -> crate::types::Image {
        crate::types::Image {
            creation_time: self.creation_time,
            description: self.description,
            display_name: self.display_name,
            failure_reason: self.failure_reason,
            image_arn: self.image_arn,
            image_name: self.image_name,
            image_status: self.image_status,
            last_modified_time: self.last_modified_time,
        }
    }
}
