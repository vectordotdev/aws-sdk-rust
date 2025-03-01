// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The KernelGateway app settings.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct KernelGatewayAppSettings {
    /// <p>The default instance type and the Amazon Resource Name (ARN) of the default SageMaker image used by the KernelGateway app.</p> <note>
    /// <p>The Amazon SageMaker Studio UI does not use the default instance type value set here. The default instance type set here is used when Apps are created using the Amazon Web Services Command Line Interface or Amazon Web Services CloudFormation and the instance type parameter value is not passed.</p>
    /// </note>
    #[doc(hidden)]
    pub default_resource_spec: ::std::option::Option<crate::types::ResourceSpec>,
    /// <p>A list of custom SageMaker images that are configured to run as a KernelGateway app.</p>
    #[doc(hidden)]
    pub custom_images: ::std::option::Option<::std::vec::Vec<crate::types::CustomImage>>,
    /// <p> The Amazon Resource Name (ARN) of the Lifecycle Configurations attached to the the user profile or domain.</p> <note>
    /// <p>To remove a Lifecycle Config, you must set <code>LifecycleConfigArns</code> to an empty list.</p>
    /// </note>
    #[doc(hidden)]
    pub lifecycle_config_arns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl KernelGatewayAppSettings {
    /// <p>The default instance type and the Amazon Resource Name (ARN) of the default SageMaker image used by the KernelGateway app.</p> <note>
    /// <p>The Amazon SageMaker Studio UI does not use the default instance type value set here. The default instance type set here is used when Apps are created using the Amazon Web Services Command Line Interface or Amazon Web Services CloudFormation and the instance type parameter value is not passed.</p>
    /// </note>
    pub fn default_resource_spec(&self) -> ::std::option::Option<&crate::types::ResourceSpec> {
        self.default_resource_spec.as_ref()
    }
    /// <p>A list of custom SageMaker images that are configured to run as a KernelGateway app.</p>
    pub fn custom_images(&self) -> ::std::option::Option<&[crate::types::CustomImage]> {
        self.custom_images.as_deref()
    }
    /// <p> The Amazon Resource Name (ARN) of the Lifecycle Configurations attached to the the user profile or domain.</p> <note>
    /// <p>To remove a Lifecycle Config, you must set <code>LifecycleConfigArns</code> to an empty list.</p>
    /// </note>
    pub fn lifecycle_config_arns(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.lifecycle_config_arns.as_deref()
    }
}
impl KernelGatewayAppSettings {
    /// Creates a new builder-style object to manufacture [`KernelGatewayAppSettings`](crate::types::KernelGatewayAppSettings).
    pub fn builder() -> crate::types::builders::KernelGatewayAppSettingsBuilder {
        crate::types::builders::KernelGatewayAppSettingsBuilder::default()
    }
}

/// A builder for [`KernelGatewayAppSettings`](crate::types::KernelGatewayAppSettings).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct KernelGatewayAppSettingsBuilder {
    pub(crate) default_resource_spec: ::std::option::Option<crate::types::ResourceSpec>,
    pub(crate) custom_images: ::std::option::Option<::std::vec::Vec<crate::types::CustomImage>>,
    pub(crate) lifecycle_config_arns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl KernelGatewayAppSettingsBuilder {
    /// <p>The default instance type and the Amazon Resource Name (ARN) of the default SageMaker image used by the KernelGateway app.</p> <note>
    /// <p>The Amazon SageMaker Studio UI does not use the default instance type value set here. The default instance type set here is used when Apps are created using the Amazon Web Services Command Line Interface or Amazon Web Services CloudFormation and the instance type parameter value is not passed.</p>
    /// </note>
    pub fn default_resource_spec(mut self, input: crate::types::ResourceSpec) -> Self {
        self.default_resource_spec = ::std::option::Option::Some(input);
        self
    }
    /// <p>The default instance type and the Amazon Resource Name (ARN) of the default SageMaker image used by the KernelGateway app.</p> <note>
    /// <p>The Amazon SageMaker Studio UI does not use the default instance type value set here. The default instance type set here is used when Apps are created using the Amazon Web Services Command Line Interface or Amazon Web Services CloudFormation and the instance type parameter value is not passed.</p>
    /// </note>
    pub fn set_default_resource_spec(
        mut self,
        input: ::std::option::Option<crate::types::ResourceSpec>,
    ) -> Self {
        self.default_resource_spec = input;
        self
    }
    /// Appends an item to `custom_images`.
    ///
    /// To override the contents of this collection use [`set_custom_images`](Self::set_custom_images).
    ///
    /// <p>A list of custom SageMaker images that are configured to run as a KernelGateway app.</p>
    pub fn custom_images(mut self, input: crate::types::CustomImage) -> Self {
        let mut v = self.custom_images.unwrap_or_default();
        v.push(input);
        self.custom_images = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of custom SageMaker images that are configured to run as a KernelGateway app.</p>
    pub fn set_custom_images(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::CustomImage>>,
    ) -> Self {
        self.custom_images = input;
        self
    }
    /// Appends an item to `lifecycle_config_arns`.
    ///
    /// To override the contents of this collection use [`set_lifecycle_config_arns`](Self::set_lifecycle_config_arns).
    ///
    /// <p> The Amazon Resource Name (ARN) of the Lifecycle Configurations attached to the the user profile or domain.</p> <note>
    /// <p>To remove a Lifecycle Config, you must set <code>LifecycleConfigArns</code> to an empty list.</p>
    /// </note>
    pub fn lifecycle_config_arns(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.lifecycle_config_arns.unwrap_or_default();
        v.push(input.into());
        self.lifecycle_config_arns = ::std::option::Option::Some(v);
        self
    }
    /// <p> The Amazon Resource Name (ARN) of the Lifecycle Configurations attached to the the user profile or domain.</p> <note>
    /// <p>To remove a Lifecycle Config, you must set <code>LifecycleConfigArns</code> to an empty list.</p>
    /// </note>
    pub fn set_lifecycle_config_arns(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.lifecycle_config_arns = input;
        self
    }
    /// Consumes the builder and constructs a [`KernelGatewayAppSettings`](crate::types::KernelGatewayAppSettings).
    pub fn build(self) -> crate::types::KernelGatewayAppSettings {
        crate::types::KernelGatewayAppSettings {
            default_resource_spec: self.default_resource_spec,
            custom_images: self.custom_images,
            lifecycle_config_arns: self.lifecycle_config_arns,
        }
    }
}
