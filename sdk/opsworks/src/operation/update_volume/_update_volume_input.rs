// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateVolumeInput {
    /// <p>The volume ID.</p>
    #[doc(hidden)]
    pub volume_id: ::std::option::Option<::std::string::String>,
    /// <p>The new name.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The new mount point.</p>
    #[doc(hidden)]
    pub mount_point: ::std::option::Option<::std::string::String>,
}
impl UpdateVolumeInput {
    /// <p>The volume ID.</p>
    pub fn volume_id(&self) -> ::std::option::Option<&str> {
        self.volume_id.as_deref()
    }
    /// <p>The new name.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The new mount point.</p>
    pub fn mount_point(&self) -> ::std::option::Option<&str> {
        self.mount_point.as_deref()
    }
}
impl UpdateVolumeInput {
    /// Creates a new builder-style object to manufacture [`UpdateVolumeInput`](crate::operation::update_volume::UpdateVolumeInput).
    pub fn builder() -> crate::operation::update_volume::builders::UpdateVolumeInputBuilder {
        crate::operation::update_volume::builders::UpdateVolumeInputBuilder::default()
    }
}

/// A builder for [`UpdateVolumeInput`](crate::operation::update_volume::UpdateVolumeInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateVolumeInputBuilder {
    pub(crate) volume_id: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) mount_point: ::std::option::Option<::std::string::String>,
}
impl UpdateVolumeInputBuilder {
    /// <p>The volume ID.</p>
    pub fn volume_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.volume_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The volume ID.</p>
    pub fn set_volume_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.volume_id = input;
        self
    }
    /// <p>The new name.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The new name.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The new mount point.</p>
    pub fn mount_point(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.mount_point = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The new mount point.</p>
    pub fn set_mount_point(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.mount_point = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateVolumeInput`](crate::operation::update_volume::UpdateVolumeInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_volume::UpdateVolumeInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::update_volume::UpdateVolumeInput {
            volume_id: self.volume_id,
            name: self.name,
            mount_point: self.mount_point,
        })
    }
}
