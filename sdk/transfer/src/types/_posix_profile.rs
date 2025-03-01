// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The full POSIX identity, including user ID (<code>Uid</code>), group ID (<code>Gid</code>), and any secondary groups IDs (<code>SecondaryGids</code>), that controls your users' access to your Amazon EFS file systems. The POSIX permissions that are set on files and directories in your file system determine the level of access your users get when transferring files into and out of your Amazon EFS file systems.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PosixProfile {
    /// <p>The POSIX user ID used for all EFS operations by this user.</p>
    #[doc(hidden)]
    pub uid: ::std::option::Option<i64>,
    /// <p>The POSIX group ID used for all EFS operations by this user.</p>
    #[doc(hidden)]
    pub gid: ::std::option::Option<i64>,
    /// <p>The secondary POSIX group IDs used for all EFS operations by this user.</p>
    #[doc(hidden)]
    pub secondary_gids: ::std::option::Option<::std::vec::Vec<i64>>,
}
impl PosixProfile {
    /// <p>The POSIX user ID used for all EFS operations by this user.</p>
    pub fn uid(&self) -> ::std::option::Option<i64> {
        self.uid
    }
    /// <p>The POSIX group ID used for all EFS operations by this user.</p>
    pub fn gid(&self) -> ::std::option::Option<i64> {
        self.gid
    }
    /// <p>The secondary POSIX group IDs used for all EFS operations by this user.</p>
    pub fn secondary_gids(&self) -> ::std::option::Option<&[i64]> {
        self.secondary_gids.as_deref()
    }
}
impl PosixProfile {
    /// Creates a new builder-style object to manufacture [`PosixProfile`](crate::types::PosixProfile).
    pub fn builder() -> crate::types::builders::PosixProfileBuilder {
        crate::types::builders::PosixProfileBuilder::default()
    }
}

/// A builder for [`PosixProfile`](crate::types::PosixProfile).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PosixProfileBuilder {
    pub(crate) uid: ::std::option::Option<i64>,
    pub(crate) gid: ::std::option::Option<i64>,
    pub(crate) secondary_gids: ::std::option::Option<::std::vec::Vec<i64>>,
}
impl PosixProfileBuilder {
    /// <p>The POSIX user ID used for all EFS operations by this user.</p>
    pub fn uid(mut self, input: i64) -> Self {
        self.uid = ::std::option::Option::Some(input);
        self
    }
    /// <p>The POSIX user ID used for all EFS operations by this user.</p>
    pub fn set_uid(mut self, input: ::std::option::Option<i64>) -> Self {
        self.uid = input;
        self
    }
    /// <p>The POSIX group ID used for all EFS operations by this user.</p>
    pub fn gid(mut self, input: i64) -> Self {
        self.gid = ::std::option::Option::Some(input);
        self
    }
    /// <p>The POSIX group ID used for all EFS operations by this user.</p>
    pub fn set_gid(mut self, input: ::std::option::Option<i64>) -> Self {
        self.gid = input;
        self
    }
    /// Appends an item to `secondary_gids`.
    ///
    /// To override the contents of this collection use [`set_secondary_gids`](Self::set_secondary_gids).
    ///
    /// <p>The secondary POSIX group IDs used for all EFS operations by this user.</p>
    pub fn secondary_gids(mut self, input: i64) -> Self {
        let mut v = self.secondary_gids.unwrap_or_default();
        v.push(input);
        self.secondary_gids = ::std::option::Option::Some(v);
        self
    }
    /// <p>The secondary POSIX group IDs used for all EFS operations by this user.</p>
    pub fn set_secondary_gids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<i64>>,
    ) -> Self {
        self.secondary_gids = input;
        self
    }
    /// Consumes the builder and constructs a [`PosixProfile`](crate::types::PosixProfile).
    pub fn build(self) -> crate::types::PosixProfile {
        crate::types::PosixProfile {
            uid: self.uid,
            gid: self.gid,
            secondary_gids: self.secondary_gids,
        }
    }
}
