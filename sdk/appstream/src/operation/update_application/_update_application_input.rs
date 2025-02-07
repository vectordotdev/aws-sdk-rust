// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateApplicationInput {
    /// <p>The name of the application. This name is visible to users when display name is not specified.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The display name of the application. This name is visible to users in the application catalog.</p>
    #[doc(hidden)]
    pub display_name: ::std::option::Option<::std::string::String>,
    /// <p>The description of the application.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The icon S3 location of the application.</p>
    #[doc(hidden)]
    pub icon_s3_location: ::std::option::Option<crate::types::S3Location>,
    /// <p>The launch path of the application.</p>
    #[doc(hidden)]
    pub launch_path: ::std::option::Option<::std::string::String>,
    /// <p>The working directory of the application.</p>
    #[doc(hidden)]
    pub working_directory: ::std::option::Option<::std::string::String>,
    /// <p>The launch parameters of the application.</p>
    #[doc(hidden)]
    pub launch_parameters: ::std::option::Option<::std::string::String>,
    /// <p>The ARN of the app block.</p>
    #[doc(hidden)]
    pub app_block_arn: ::std::option::Option<::std::string::String>,
    /// <p>The attributes to delete for an application.</p>
    #[doc(hidden)]
    pub attributes_to_delete:
        ::std::option::Option<::std::vec::Vec<crate::types::ApplicationAttribute>>,
}
impl UpdateApplicationInput {
    /// <p>The name of the application. This name is visible to users when display name is not specified.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The display name of the application. This name is visible to users in the application catalog.</p>
    pub fn display_name(&self) -> ::std::option::Option<&str> {
        self.display_name.as_deref()
    }
    /// <p>The description of the application.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The icon S3 location of the application.</p>
    pub fn icon_s3_location(&self) -> ::std::option::Option<&crate::types::S3Location> {
        self.icon_s3_location.as_ref()
    }
    /// <p>The launch path of the application.</p>
    pub fn launch_path(&self) -> ::std::option::Option<&str> {
        self.launch_path.as_deref()
    }
    /// <p>The working directory of the application.</p>
    pub fn working_directory(&self) -> ::std::option::Option<&str> {
        self.working_directory.as_deref()
    }
    /// <p>The launch parameters of the application.</p>
    pub fn launch_parameters(&self) -> ::std::option::Option<&str> {
        self.launch_parameters.as_deref()
    }
    /// <p>The ARN of the app block.</p>
    pub fn app_block_arn(&self) -> ::std::option::Option<&str> {
        self.app_block_arn.as_deref()
    }
    /// <p>The attributes to delete for an application.</p>
    pub fn attributes_to_delete(
        &self,
    ) -> ::std::option::Option<&[crate::types::ApplicationAttribute]> {
        self.attributes_to_delete.as_deref()
    }
}
impl UpdateApplicationInput {
    /// Creates a new builder-style object to manufacture [`UpdateApplicationInput`](crate::operation::update_application::UpdateApplicationInput).
    pub fn builder() -> crate::operation::update_application::builders::UpdateApplicationInputBuilder
    {
        crate::operation::update_application::builders::UpdateApplicationInputBuilder::default()
    }
}

/// A builder for [`UpdateApplicationInput`](crate::operation::update_application::UpdateApplicationInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateApplicationInputBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) display_name: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) icon_s3_location: ::std::option::Option<crate::types::S3Location>,
    pub(crate) launch_path: ::std::option::Option<::std::string::String>,
    pub(crate) working_directory: ::std::option::Option<::std::string::String>,
    pub(crate) launch_parameters: ::std::option::Option<::std::string::String>,
    pub(crate) app_block_arn: ::std::option::Option<::std::string::String>,
    pub(crate) attributes_to_delete:
        ::std::option::Option<::std::vec::Vec<crate::types::ApplicationAttribute>>,
}
impl UpdateApplicationInputBuilder {
    /// <p>The name of the application. This name is visible to users when display name is not specified.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the application. This name is visible to users when display name is not specified.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The display name of the application. This name is visible to users in the application catalog.</p>
    pub fn display_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.display_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The display name of the application. This name is visible to users in the application catalog.</p>
    pub fn set_display_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.display_name = input;
        self
    }
    /// <p>The description of the application.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description of the application.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The icon S3 location of the application.</p>
    pub fn icon_s3_location(mut self, input: crate::types::S3Location) -> Self {
        self.icon_s3_location = ::std::option::Option::Some(input);
        self
    }
    /// <p>The icon S3 location of the application.</p>
    pub fn set_icon_s3_location(
        mut self,
        input: ::std::option::Option<crate::types::S3Location>,
    ) -> Self {
        self.icon_s3_location = input;
        self
    }
    /// <p>The launch path of the application.</p>
    pub fn launch_path(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.launch_path = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The launch path of the application.</p>
    pub fn set_launch_path(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.launch_path = input;
        self
    }
    /// <p>The working directory of the application.</p>
    pub fn working_directory(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.working_directory = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The working directory of the application.</p>
    pub fn set_working_directory(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.working_directory = input;
        self
    }
    /// <p>The launch parameters of the application.</p>
    pub fn launch_parameters(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.launch_parameters = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The launch parameters of the application.</p>
    pub fn set_launch_parameters(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.launch_parameters = input;
        self
    }
    /// <p>The ARN of the app block.</p>
    pub fn app_block_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.app_block_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the app block.</p>
    pub fn set_app_block_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.app_block_arn = input;
        self
    }
    /// Appends an item to `attributes_to_delete`.
    ///
    /// To override the contents of this collection use [`set_attributes_to_delete`](Self::set_attributes_to_delete).
    ///
    /// <p>The attributes to delete for an application.</p>
    pub fn attributes_to_delete(mut self, input: crate::types::ApplicationAttribute) -> Self {
        let mut v = self.attributes_to_delete.unwrap_or_default();
        v.push(input);
        self.attributes_to_delete = ::std::option::Option::Some(v);
        self
    }
    /// <p>The attributes to delete for an application.</p>
    pub fn set_attributes_to_delete(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ApplicationAttribute>>,
    ) -> Self {
        self.attributes_to_delete = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateApplicationInput`](crate::operation::update_application::UpdateApplicationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_application::UpdateApplicationInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::update_application::UpdateApplicationInput {
                name: self.name,
                display_name: self.display_name,
                description: self.description,
                icon_s3_location: self.icon_s3_location,
                launch_path: self.launch_path,
                working_directory: self.working_directory,
                launch_parameters: self.launch_parameters,
                app_block_arn: self.app_block_arn,
                attributes_to_delete: self.attributes_to_delete,
            },
        )
    }
}
