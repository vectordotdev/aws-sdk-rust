// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a block storage disk mapping.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DiskMap {
    /// <p>The original disk path exposed to the instance (for example, <code>/dev/sdh</code>).</p>
    #[doc(hidden)]
    pub original_disk_path: ::std::option::Option<::std::string::String>,
    /// <p>The new disk name (e.g., <code>my-new-disk</code>).</p>
    #[doc(hidden)]
    pub new_disk_name: ::std::option::Option<::std::string::String>,
}
impl DiskMap {
    /// <p>The original disk path exposed to the instance (for example, <code>/dev/sdh</code>).</p>
    pub fn original_disk_path(&self) -> ::std::option::Option<&str> {
        self.original_disk_path.as_deref()
    }
    /// <p>The new disk name (e.g., <code>my-new-disk</code>).</p>
    pub fn new_disk_name(&self) -> ::std::option::Option<&str> {
        self.new_disk_name.as_deref()
    }
}
impl DiskMap {
    /// Creates a new builder-style object to manufacture [`DiskMap`](crate::types::DiskMap).
    pub fn builder() -> crate::types::builders::DiskMapBuilder {
        crate::types::builders::DiskMapBuilder::default()
    }
}

/// A builder for [`DiskMap`](crate::types::DiskMap).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DiskMapBuilder {
    pub(crate) original_disk_path: ::std::option::Option<::std::string::String>,
    pub(crate) new_disk_name: ::std::option::Option<::std::string::String>,
}
impl DiskMapBuilder {
    /// <p>The original disk path exposed to the instance (for example, <code>/dev/sdh</code>).</p>
    pub fn original_disk_path(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.original_disk_path = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The original disk path exposed to the instance (for example, <code>/dev/sdh</code>).</p>
    pub fn set_original_disk_path(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.original_disk_path = input;
        self
    }
    /// <p>The new disk name (e.g., <code>my-new-disk</code>).</p>
    pub fn new_disk_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.new_disk_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The new disk name (e.g., <code>my-new-disk</code>).</p>
    pub fn set_new_disk_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.new_disk_name = input;
        self
    }
    /// Consumes the builder and constructs a [`DiskMap`](crate::types::DiskMap).
    pub fn build(self) -> crate::types::DiskMap {
        crate::types::DiskMap {
            original_disk_path: self.original_disk_path,
            new_disk_name: self.new_disk_name,
        }
    }
}
