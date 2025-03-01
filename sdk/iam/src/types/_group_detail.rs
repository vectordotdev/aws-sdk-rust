// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about an IAM group, including all of the group's policies.</p>
/// <p>This data type is used as a response element in the <code>GetAccountAuthorizationDetails</code> operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GroupDetail {
    /// <p>The path to the group. For more information about paths, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">IAM identifiers</a> in the <i>IAM User Guide</i>.</p>
    #[doc(hidden)]
    pub path: ::std::option::Option<::std::string::String>,
    /// <p>The friendly name that identifies the group.</p>
    #[doc(hidden)]
    pub group_name: ::std::option::Option<::std::string::String>,
    /// <p>The stable and unique string identifying the group. For more information about IDs, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">IAM identifiers</a> in the <i>IAM User Guide</i>.</p>
    #[doc(hidden)]
    pub group_id: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN). ARNs are unique identifiers for Amazon Web Services resources.</p>
    /// <p>For more information about ARNs, go to <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>. </p>
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>The date and time, in <a href="http://www.iso.org/iso/iso8601">ISO 8601 date-time format</a>, when the group was created.</p>
    #[doc(hidden)]
    pub create_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>A list of the inline policies embedded in the group.</p>
    #[doc(hidden)]
    pub group_policy_list: ::std::option::Option<::std::vec::Vec<crate::types::PolicyDetail>>,
    /// <p>A list of the managed policies attached to the group.</p>
    #[doc(hidden)]
    pub attached_managed_policies:
        ::std::option::Option<::std::vec::Vec<crate::types::AttachedPolicy>>,
}
impl GroupDetail {
    /// <p>The path to the group. For more information about paths, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">IAM identifiers</a> in the <i>IAM User Guide</i>.</p>
    pub fn path(&self) -> ::std::option::Option<&str> {
        self.path.as_deref()
    }
    /// <p>The friendly name that identifies the group.</p>
    pub fn group_name(&self) -> ::std::option::Option<&str> {
        self.group_name.as_deref()
    }
    /// <p>The stable and unique string identifying the group. For more information about IDs, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">IAM identifiers</a> in the <i>IAM User Guide</i>.</p>
    pub fn group_id(&self) -> ::std::option::Option<&str> {
        self.group_id.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN). ARNs are unique identifiers for Amazon Web Services resources.</p>
    /// <p>For more information about ARNs, go to <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>. </p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>The date and time, in <a href="http://www.iso.org/iso/iso8601">ISO 8601 date-time format</a>, when the group was created.</p>
    pub fn create_date(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.create_date.as_ref()
    }
    /// <p>A list of the inline policies embedded in the group.</p>
    pub fn group_policy_list(&self) -> ::std::option::Option<&[crate::types::PolicyDetail]> {
        self.group_policy_list.as_deref()
    }
    /// <p>A list of the managed policies attached to the group.</p>
    pub fn attached_managed_policies(
        &self,
    ) -> ::std::option::Option<&[crate::types::AttachedPolicy]> {
        self.attached_managed_policies.as_deref()
    }
}
impl GroupDetail {
    /// Creates a new builder-style object to manufacture [`GroupDetail`](crate::types::GroupDetail).
    pub fn builder() -> crate::types::builders::GroupDetailBuilder {
        crate::types::builders::GroupDetailBuilder::default()
    }
}

/// A builder for [`GroupDetail`](crate::types::GroupDetail).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GroupDetailBuilder {
    pub(crate) path: ::std::option::Option<::std::string::String>,
    pub(crate) group_name: ::std::option::Option<::std::string::String>,
    pub(crate) group_id: ::std::option::Option<::std::string::String>,
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) create_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) group_policy_list:
        ::std::option::Option<::std::vec::Vec<crate::types::PolicyDetail>>,
    pub(crate) attached_managed_policies:
        ::std::option::Option<::std::vec::Vec<crate::types::AttachedPolicy>>,
}
impl GroupDetailBuilder {
    /// <p>The path to the group. For more information about paths, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">IAM identifiers</a> in the <i>IAM User Guide</i>.</p>
    pub fn path(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.path = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The path to the group. For more information about paths, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">IAM identifiers</a> in the <i>IAM User Guide</i>.</p>
    pub fn set_path(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.path = input;
        self
    }
    /// <p>The friendly name that identifies the group.</p>
    pub fn group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.group_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The friendly name that identifies the group.</p>
    pub fn set_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.group_name = input;
        self
    }
    /// <p>The stable and unique string identifying the group. For more information about IDs, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">IAM identifiers</a> in the <i>IAM User Guide</i>.</p>
    pub fn group_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.group_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The stable and unique string identifying the group. For more information about IDs, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_Identifiers.html">IAM identifiers</a> in the <i>IAM User Guide</i>.</p>
    pub fn set_group_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.group_id = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN). ARNs are unique identifiers for Amazon Web Services resources.</p>
    /// <p>For more information about ARNs, go to <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>. </p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN). ARNs are unique identifiers for Amazon Web Services resources.</p>
    /// <p>For more information about ARNs, go to <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>. </p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The date and time, in <a href="http://www.iso.org/iso/iso8601">ISO 8601 date-time format</a>, when the group was created.</p>
    pub fn create_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.create_date = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time, in <a href="http://www.iso.org/iso/iso8601">ISO 8601 date-time format</a>, when the group was created.</p>
    pub fn set_create_date(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.create_date = input;
        self
    }
    /// Appends an item to `group_policy_list`.
    ///
    /// To override the contents of this collection use [`set_group_policy_list`](Self::set_group_policy_list).
    ///
    /// <p>A list of the inline policies embedded in the group.</p>
    pub fn group_policy_list(mut self, input: crate::types::PolicyDetail) -> Self {
        let mut v = self.group_policy_list.unwrap_or_default();
        v.push(input);
        self.group_policy_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of the inline policies embedded in the group.</p>
    pub fn set_group_policy_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::PolicyDetail>>,
    ) -> Self {
        self.group_policy_list = input;
        self
    }
    /// Appends an item to `attached_managed_policies`.
    ///
    /// To override the contents of this collection use [`set_attached_managed_policies`](Self::set_attached_managed_policies).
    ///
    /// <p>A list of the managed policies attached to the group.</p>
    pub fn attached_managed_policies(mut self, input: crate::types::AttachedPolicy) -> Self {
        let mut v = self.attached_managed_policies.unwrap_or_default();
        v.push(input);
        self.attached_managed_policies = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of the managed policies attached to the group.</p>
    pub fn set_attached_managed_policies(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::AttachedPolicy>>,
    ) -> Self {
        self.attached_managed_policies = input;
        self
    }
    /// Consumes the builder and constructs a [`GroupDetail`](crate::types::GroupDetail).
    pub fn build(self) -> crate::types::GroupDetail {
        crate::types::GroupDetail {
            path: self.path,
            group_name: self.group_name,
            group_id: self.group_id,
            arn: self.arn,
            create_date: self.create_date,
            group_policy_list: self.group_policy_list,
            attached_managed_policies: self.attached_managed_policies,
        }
    }
}
