// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateAppImageConfigInput {
    /// <p>The name of the AppImageConfig to update.</p>
    #[doc(hidden)]
    pub app_image_config_name: ::std::option::Option<::std::string::String>,
    /// <p>The new KernelGateway app to run on the image.</p>
    #[doc(hidden)]
    pub kernel_gateway_image_config: ::std::option::Option<crate::types::KernelGatewayImageConfig>,
}
impl UpdateAppImageConfigInput {
    /// <p>The name of the AppImageConfig to update.</p>
    pub fn app_image_config_name(&self) -> ::std::option::Option<&str> {
        self.app_image_config_name.as_deref()
    }
    /// <p>The new KernelGateway app to run on the image.</p>
    pub fn kernel_gateway_image_config(
        &self,
    ) -> ::std::option::Option<&crate::types::KernelGatewayImageConfig> {
        self.kernel_gateway_image_config.as_ref()
    }
}
impl UpdateAppImageConfigInput {
    /// Creates a new builder-style object to manufacture [`UpdateAppImageConfigInput`](crate::operation::update_app_image_config::UpdateAppImageConfigInput).
    pub fn builder(
    ) -> crate::operation::update_app_image_config::builders::UpdateAppImageConfigInputBuilder {
        crate::operation::update_app_image_config::builders::UpdateAppImageConfigInputBuilder::default()
    }
}

/// A builder for [`UpdateAppImageConfigInput`](crate::operation::update_app_image_config::UpdateAppImageConfigInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateAppImageConfigInputBuilder {
    pub(crate) app_image_config_name: ::std::option::Option<::std::string::String>,
    pub(crate) kernel_gateway_image_config:
        ::std::option::Option<crate::types::KernelGatewayImageConfig>,
}
impl UpdateAppImageConfigInputBuilder {
    /// <p>The name of the AppImageConfig to update.</p>
    pub fn app_image_config_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.app_image_config_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the AppImageConfig to update.</p>
    pub fn set_app_image_config_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.app_image_config_name = input;
        self
    }
    /// <p>The new KernelGateway app to run on the image.</p>
    pub fn kernel_gateway_image_config(
        mut self,
        input: crate::types::KernelGatewayImageConfig,
    ) -> Self {
        self.kernel_gateway_image_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>The new KernelGateway app to run on the image.</p>
    pub fn set_kernel_gateway_image_config(
        mut self,
        input: ::std::option::Option<crate::types::KernelGatewayImageConfig>,
    ) -> Self {
        self.kernel_gateway_image_config = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateAppImageConfigInput`](crate::operation::update_app_image_config::UpdateAppImageConfigInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_app_image_config::UpdateAppImageConfigInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::update_app_image_config::UpdateAppImageConfigInput {
                app_image_config_name: self.app_image_config_name,
                kernel_gateway_image_config: self.kernel_gateway_image_config,
            },
        )
    }
}
