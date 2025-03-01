// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct StartFailbackLaunchInput {
    /// <p>The IDs of the Recovery Instance whose failback launch we want to request.</p>
    #[doc(hidden)]
    pub recovery_instance_i_ds: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The tags to be associated with the failback launch Job.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl StartFailbackLaunchInput {
    /// <p>The IDs of the Recovery Instance whose failback launch we want to request.</p>
    pub fn recovery_instance_i_ds(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.recovery_instance_i_ds.as_deref()
    }
    /// <p>The tags to be associated with the failback launch Job.</p>
    pub fn tags(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.tags.as_ref()
    }
}
impl ::std::fmt::Debug for StartFailbackLaunchInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("StartFailbackLaunchInput");
        formatter.field("recovery_instance_i_ds", &self.recovery_instance_i_ds);
        formatter.field("tags", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl StartFailbackLaunchInput {
    /// Creates a new builder-style object to manufacture [`StartFailbackLaunchInput`](crate::operation::start_failback_launch::StartFailbackLaunchInput).
    pub fn builder(
    ) -> crate::operation::start_failback_launch::builders::StartFailbackLaunchInputBuilder {
        crate::operation::start_failback_launch::builders::StartFailbackLaunchInputBuilder::default(
        )
    }
}

/// A builder for [`StartFailbackLaunchInput`](crate::operation::start_failback_launch::StartFailbackLaunchInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct StartFailbackLaunchInputBuilder {
    pub(crate) recovery_instance_i_ds:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl StartFailbackLaunchInputBuilder {
    /// Appends an item to `recovery_instance_i_ds`.
    ///
    /// To override the contents of this collection use [`set_recovery_instance_i_ds`](Self::set_recovery_instance_i_ds).
    ///
    /// <p>The IDs of the Recovery Instance whose failback launch we want to request.</p>
    pub fn recovery_instance_i_ds(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.recovery_instance_i_ds.unwrap_or_default();
        v.push(input.into());
        self.recovery_instance_i_ds = ::std::option::Option::Some(v);
        self
    }
    /// <p>The IDs of the Recovery Instance whose failback launch we want to request.</p>
    pub fn set_recovery_instance_i_ds(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.recovery_instance_i_ds = input;
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags to be associated with the failback launch Job.</p>
    pub fn tags(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>The tags to be associated with the failback launch Job.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`StartFailbackLaunchInput`](crate::operation::start_failback_launch::StartFailbackLaunchInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::start_failback_launch::StartFailbackLaunchInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::start_failback_launch::StartFailbackLaunchInput {
                recovery_instance_i_ds: self.recovery_instance_i_ds,
                tags: self.tags,
            },
        )
    }
}
impl ::std::fmt::Debug for StartFailbackLaunchInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("StartFailbackLaunchInputBuilder");
        formatter.field("recovery_instance_i_ds", &self.recovery_instance_i_ds);
        formatter.field("tags", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
