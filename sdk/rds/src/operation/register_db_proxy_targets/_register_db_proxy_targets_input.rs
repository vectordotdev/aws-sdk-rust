// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RegisterDbProxyTargetsInput {
    /// <p>The identifier of the <code>DBProxy</code> that is associated with the <code>DBProxyTargetGroup</code>.</p>
    #[doc(hidden)]
    pub db_proxy_name: ::std::option::Option<::std::string::String>,
    /// <p>The identifier of the <code>DBProxyTargetGroup</code>.</p>
    #[doc(hidden)]
    pub target_group_name: ::std::option::Option<::std::string::String>,
    /// <p>One or more DB instance identifiers.</p>
    #[doc(hidden)]
    pub db_instance_identifiers: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>One or more DB cluster identifiers.</p>
    #[doc(hidden)]
    pub db_cluster_identifiers: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl RegisterDbProxyTargetsInput {
    /// <p>The identifier of the <code>DBProxy</code> that is associated with the <code>DBProxyTargetGroup</code>.</p>
    pub fn db_proxy_name(&self) -> ::std::option::Option<&str> {
        self.db_proxy_name.as_deref()
    }
    /// <p>The identifier of the <code>DBProxyTargetGroup</code>.</p>
    pub fn target_group_name(&self) -> ::std::option::Option<&str> {
        self.target_group_name.as_deref()
    }
    /// <p>One or more DB instance identifiers.</p>
    pub fn db_instance_identifiers(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.db_instance_identifiers.as_deref()
    }
    /// <p>One or more DB cluster identifiers.</p>
    pub fn db_cluster_identifiers(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.db_cluster_identifiers.as_deref()
    }
}
impl RegisterDbProxyTargetsInput {
    /// Creates a new builder-style object to manufacture [`RegisterDbProxyTargetsInput`](crate::operation::register_db_proxy_targets::RegisterDbProxyTargetsInput).
    pub fn builder(
    ) -> crate::operation::register_db_proxy_targets::builders::RegisterDbProxyTargetsInputBuilder
    {
        crate::operation::register_db_proxy_targets::builders::RegisterDbProxyTargetsInputBuilder::default()
    }
}

/// A builder for [`RegisterDbProxyTargetsInput`](crate::operation::register_db_proxy_targets::RegisterDbProxyTargetsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RegisterDbProxyTargetsInputBuilder {
    pub(crate) db_proxy_name: ::std::option::Option<::std::string::String>,
    pub(crate) target_group_name: ::std::option::Option<::std::string::String>,
    pub(crate) db_instance_identifiers:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) db_cluster_identifiers:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl RegisterDbProxyTargetsInputBuilder {
    /// <p>The identifier of the <code>DBProxy</code> that is associated with the <code>DBProxyTargetGroup</code>.</p>
    pub fn db_proxy_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.db_proxy_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the <code>DBProxy</code> that is associated with the <code>DBProxyTargetGroup</code>.</p>
    pub fn set_db_proxy_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.db_proxy_name = input;
        self
    }
    /// <p>The identifier of the <code>DBProxyTargetGroup</code>.</p>
    pub fn target_group_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.target_group_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of the <code>DBProxyTargetGroup</code>.</p>
    pub fn set_target_group_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.target_group_name = input;
        self
    }
    /// Appends an item to `db_instance_identifiers`.
    ///
    /// To override the contents of this collection use [`set_db_instance_identifiers`](Self::set_db_instance_identifiers).
    ///
    /// <p>One or more DB instance identifiers.</p>
    pub fn db_instance_identifiers(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.db_instance_identifiers.unwrap_or_default();
        v.push(input.into());
        self.db_instance_identifiers = ::std::option::Option::Some(v);
        self
    }
    /// <p>One or more DB instance identifiers.</p>
    pub fn set_db_instance_identifiers(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.db_instance_identifiers = input;
        self
    }
    /// Appends an item to `db_cluster_identifiers`.
    ///
    /// To override the contents of this collection use [`set_db_cluster_identifiers`](Self::set_db_cluster_identifiers).
    ///
    /// <p>One or more DB cluster identifiers.</p>
    pub fn db_cluster_identifiers(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.db_cluster_identifiers.unwrap_or_default();
        v.push(input.into());
        self.db_cluster_identifiers = ::std::option::Option::Some(v);
        self
    }
    /// <p>One or more DB cluster identifiers.</p>
    pub fn set_db_cluster_identifiers(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.db_cluster_identifiers = input;
        self
    }
    /// Consumes the builder and constructs a [`RegisterDbProxyTargetsInput`](crate::operation::register_db_proxy_targets::RegisterDbProxyTargetsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::register_db_proxy_targets::RegisterDbProxyTargetsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::register_db_proxy_targets::RegisterDbProxyTargetsInput {
                db_proxy_name: self.db_proxy_name,
                target_group_name: self.target_group_name,
                db_instance_identifiers: self.db_instance_identifiers,
                db_cluster_identifiers: self.db_cluster_identifiers,
            },
        )
    }
}
