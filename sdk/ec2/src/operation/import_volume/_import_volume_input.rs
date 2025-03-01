// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ImportVolumeInput {
    /// <p>The Availability Zone for the resulting EBS volume.</p>
    #[doc(hidden)]
    pub availability_zone: ::std::option::Option<::std::string::String>,
    /// <p>A description of the volume.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: ::std::option::Option<bool>,
    /// <p>The disk image.</p>
    #[doc(hidden)]
    pub image: ::std::option::Option<crate::types::DiskImageDetail>,
    /// <p>The volume size.</p>
    #[doc(hidden)]
    pub volume: ::std::option::Option<crate::types::VolumeDetail>,
}
impl ImportVolumeInput {
    /// <p>The Availability Zone for the resulting EBS volume.</p>
    pub fn availability_zone(&self) -> ::std::option::Option<&str> {
        self.availability_zone.as_deref()
    }
    /// <p>A description of the volume.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The disk image.</p>
    pub fn image(&self) -> ::std::option::Option<&crate::types::DiskImageDetail> {
        self.image.as_ref()
    }
    /// <p>The volume size.</p>
    pub fn volume(&self) -> ::std::option::Option<&crate::types::VolumeDetail> {
        self.volume.as_ref()
    }
}
impl ImportVolumeInput {
    /// Creates a new builder-style object to manufacture [`ImportVolumeInput`](crate::operation::import_volume::ImportVolumeInput).
    pub fn builder() -> crate::operation::import_volume::builders::ImportVolumeInputBuilder {
        crate::operation::import_volume::builders::ImportVolumeInputBuilder::default()
    }
}

/// A builder for [`ImportVolumeInput`](crate::operation::import_volume::ImportVolumeInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ImportVolumeInputBuilder {
    pub(crate) availability_zone: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) image: ::std::option::Option<crate::types::DiskImageDetail>,
    pub(crate) volume: ::std::option::Option<crate::types::VolumeDetail>,
}
impl ImportVolumeInputBuilder {
    /// <p>The Availability Zone for the resulting EBS volume.</p>
    pub fn availability_zone(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.availability_zone = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Availability Zone for the resulting EBS volume.</p>
    pub fn set_availability_zone(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.availability_zone = input;
        self
    }
    /// <p>A description of the volume.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A description of the volume.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = ::std::option::Option::Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// <p>The disk image.</p>
    pub fn image(mut self, input: crate::types::DiskImageDetail) -> Self {
        self.image = ::std::option::Option::Some(input);
        self
    }
    /// <p>The disk image.</p>
    pub fn set_image(
        mut self,
        input: ::std::option::Option<crate::types::DiskImageDetail>,
    ) -> Self {
        self.image = input;
        self
    }
    /// <p>The volume size.</p>
    pub fn volume(mut self, input: crate::types::VolumeDetail) -> Self {
        self.volume = ::std::option::Option::Some(input);
        self
    }
    /// <p>The volume size.</p>
    pub fn set_volume(mut self, input: ::std::option::Option<crate::types::VolumeDetail>) -> Self {
        self.volume = input;
        self
    }
    /// Consumes the builder and constructs a [`ImportVolumeInput`](crate::operation::import_volume::ImportVolumeInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::import_volume::ImportVolumeInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::import_volume::ImportVolumeInput {
            availability_zone: self.availability_zone,
            description: self.description,
            dry_run: self.dry_run,
            image: self.image,
            volume: self.volume,
        })
    }
}
