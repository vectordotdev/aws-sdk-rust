// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies the method and snapshot to use when restarting an application using previously saved application state.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ApplicationRestoreConfiguration {
    /// <p>Specifies how the application should be restored.</p>
    #[doc(hidden)]
    pub application_restore_type: ::std::option::Option<crate::types::ApplicationRestoreType>,
    /// <p>The identifier of an existing snapshot of application state to use to restart an application. The application uses this value if <code>RESTORE_FROM_CUSTOM_SNAPSHOT</code> is specified for the <code>ApplicationRestoreType</code>.</p>
    #[doc(hidden)]
    pub snapshot_name: ::std::option::Option<::std::string::String>,
}
impl ApplicationRestoreConfiguration {
    /// <p>Specifies how the application should be restored.</p>
    pub fn application_restore_type(
        &self,
    ) -> ::std::option::Option<&crate::types::ApplicationRestoreType> {
        self.application_restore_type.as_ref()
    }
    /// <p>The identifier of an existing snapshot of application state to use to restart an application. The application uses this value if <code>RESTORE_FROM_CUSTOM_SNAPSHOT</code> is specified for the <code>ApplicationRestoreType</code>.</p>
    pub fn snapshot_name(&self) -> ::std::option::Option<&str> {
        self.snapshot_name.as_deref()
    }
}
impl ApplicationRestoreConfiguration {
    /// Creates a new builder-style object to manufacture [`ApplicationRestoreConfiguration`](crate::types::ApplicationRestoreConfiguration).
    pub fn builder() -> crate::types::builders::ApplicationRestoreConfigurationBuilder {
        crate::types::builders::ApplicationRestoreConfigurationBuilder::default()
    }
}

/// A builder for [`ApplicationRestoreConfiguration`](crate::types::ApplicationRestoreConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ApplicationRestoreConfigurationBuilder {
    pub(crate) application_restore_type:
        ::std::option::Option<crate::types::ApplicationRestoreType>,
    pub(crate) snapshot_name: ::std::option::Option<::std::string::String>,
}
impl ApplicationRestoreConfigurationBuilder {
    /// <p>Specifies how the application should be restored.</p>
    pub fn application_restore_type(mut self, input: crate::types::ApplicationRestoreType) -> Self {
        self.application_restore_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies how the application should be restored.</p>
    pub fn set_application_restore_type(
        mut self,
        input: ::std::option::Option<crate::types::ApplicationRestoreType>,
    ) -> Self {
        self.application_restore_type = input;
        self
    }
    /// <p>The identifier of an existing snapshot of application state to use to restart an application. The application uses this value if <code>RESTORE_FROM_CUSTOM_SNAPSHOT</code> is specified for the <code>ApplicationRestoreType</code>.</p>
    pub fn snapshot_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.snapshot_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of an existing snapshot of application state to use to restart an application. The application uses this value if <code>RESTORE_FROM_CUSTOM_SNAPSHOT</code> is specified for the <code>ApplicationRestoreType</code>.</p>
    pub fn set_snapshot_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.snapshot_name = input;
        self
    }
    /// Consumes the builder and constructs a [`ApplicationRestoreConfiguration`](crate::types::ApplicationRestoreConfiguration).
    pub fn build(self) -> crate::types::ApplicationRestoreConfiguration {
        crate::types::ApplicationRestoreConfiguration {
            application_restore_type: self.application_restore_type,
            snapshot_name: self.snapshot_name,
        }
    }
}
