// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Configures how streaming sessions are backed up when launched from this launch profile.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StreamConfigurationSessionBackup {
    /// <p>Specifies how artists sessions are backed up.</p>
    /// <p>Configures backups for streaming sessions launched with this launch profile. The default value is <code>DEACTIVATED</code>, which means that backups are deactivated. To allow backups, set this value to <code>AUTOMATIC</code>.</p>
    #[doc(hidden)]
    pub mode: ::std::option::Option<crate::types::SessionBackupMode>,
    /// <p>The maximum number of backups that each streaming session created from this launch profile can have.</p>
    #[doc(hidden)]
    pub max_backups_to_retain: i32,
}
impl StreamConfigurationSessionBackup {
    /// <p>Specifies how artists sessions are backed up.</p>
    /// <p>Configures backups for streaming sessions launched with this launch profile. The default value is <code>DEACTIVATED</code>, which means that backups are deactivated. To allow backups, set this value to <code>AUTOMATIC</code>.</p>
    pub fn mode(&self) -> ::std::option::Option<&crate::types::SessionBackupMode> {
        self.mode.as_ref()
    }
    /// <p>The maximum number of backups that each streaming session created from this launch profile can have.</p>
    pub fn max_backups_to_retain(&self) -> i32 {
        self.max_backups_to_retain
    }
}
impl StreamConfigurationSessionBackup {
    /// Creates a new builder-style object to manufacture [`StreamConfigurationSessionBackup`](crate::types::StreamConfigurationSessionBackup).
    pub fn builder() -> crate::types::builders::StreamConfigurationSessionBackupBuilder {
        crate::types::builders::StreamConfigurationSessionBackupBuilder::default()
    }
}

/// A builder for [`StreamConfigurationSessionBackup`](crate::types::StreamConfigurationSessionBackup).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct StreamConfigurationSessionBackupBuilder {
    pub(crate) mode: ::std::option::Option<crate::types::SessionBackupMode>,
    pub(crate) max_backups_to_retain: ::std::option::Option<i32>,
}
impl StreamConfigurationSessionBackupBuilder {
    /// <p>Specifies how artists sessions are backed up.</p>
    /// <p>Configures backups for streaming sessions launched with this launch profile. The default value is <code>DEACTIVATED</code>, which means that backups are deactivated. To allow backups, set this value to <code>AUTOMATIC</code>.</p>
    pub fn mode(mut self, input: crate::types::SessionBackupMode) -> Self {
        self.mode = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies how artists sessions are backed up.</p>
    /// <p>Configures backups for streaming sessions launched with this launch profile. The default value is <code>DEACTIVATED</code>, which means that backups are deactivated. To allow backups, set this value to <code>AUTOMATIC</code>.</p>
    pub fn set_mode(
        mut self,
        input: ::std::option::Option<crate::types::SessionBackupMode>,
    ) -> Self {
        self.mode = input;
        self
    }
    /// <p>The maximum number of backups that each streaming session created from this launch profile can have.</p>
    pub fn max_backups_to_retain(mut self, input: i32) -> Self {
        self.max_backups_to_retain = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of backups that each streaming session created from this launch profile can have.</p>
    pub fn set_max_backups_to_retain(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_backups_to_retain = input;
        self
    }
    /// Consumes the builder and constructs a [`StreamConfigurationSessionBackup`](crate::types::StreamConfigurationSessionBackup).
    pub fn build(self) -> crate::types::StreamConfigurationSessionBackup {
        crate::types::StreamConfigurationSessionBackup {
            mode: self.mode,
            max_backups_to_retain: self.max_backups_to_retain.unwrap_or_default(),
        }
    }
}
