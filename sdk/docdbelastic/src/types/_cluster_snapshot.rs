// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Returns information about a specific Elastic DocumentDB snapshot.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ClusterSnapshot {
    /// <p>A list of the IDs of subnets associated with the DB cluster snapshot.</p>
    #[doc(hidden)]
    pub subnet_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The name of the Elastic DocumentDB snapshot.</p>
    #[doc(hidden)]
    pub snapshot_name: ::std::option::Option<::std::string::String>,
    /// <p>The arn of the Elastic DocumentDB snapshot</p>
    #[doc(hidden)]
    pub snapshot_arn: ::std::option::Option<::std::string::String>,
    /// <p>The time when the Elastic DocumentDB snapshot was created in Universal Coordinated Time (UTC).</p>
    #[doc(hidden)]
    pub snapshot_creation_time: ::std::option::Option<::std::string::String>,
    /// <p>The arn of the Elastic DocumentDB cluster.</p>
    #[doc(hidden)]
    pub cluster_arn: ::std::option::Option<::std::string::String>,
    /// <p>The time when the Elastic DocumentDB cluster was created in Universal Coordinated Time (UTC).</p>
    #[doc(hidden)]
    pub cluster_creation_time: ::std::option::Option<::std::string::String>,
    /// <p>The status of the Elastic DocumentDB snapshot.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::Status>,
    /// <p>A list of the IDs of the VPC security groups associated with the cluster snapshot.</p>
    #[doc(hidden)]
    pub vpc_security_group_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The name of the Elastic DocumentDB cluster administrator.</p>
    #[doc(hidden)]
    pub admin_user_name: ::std::option::Option<::std::string::String>,
    /// <p>The KMS key identifier to use to encrypt the Elastic DocumentDB cluster.</p>
    #[doc(hidden)]
    pub kms_key_id: ::std::option::Option<::std::string::String>,
}
impl ClusterSnapshot {
    /// <p>A list of the IDs of subnets associated with the DB cluster snapshot.</p>
    pub fn subnet_ids(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.subnet_ids.as_deref()
    }
    /// <p>The name of the Elastic DocumentDB snapshot.</p>
    pub fn snapshot_name(&self) -> ::std::option::Option<&str> {
        self.snapshot_name.as_deref()
    }
    /// <p>The arn of the Elastic DocumentDB snapshot</p>
    pub fn snapshot_arn(&self) -> ::std::option::Option<&str> {
        self.snapshot_arn.as_deref()
    }
    /// <p>The time when the Elastic DocumentDB snapshot was created in Universal Coordinated Time (UTC).</p>
    pub fn snapshot_creation_time(&self) -> ::std::option::Option<&str> {
        self.snapshot_creation_time.as_deref()
    }
    /// <p>The arn of the Elastic DocumentDB cluster.</p>
    pub fn cluster_arn(&self) -> ::std::option::Option<&str> {
        self.cluster_arn.as_deref()
    }
    /// <p>The time when the Elastic DocumentDB cluster was created in Universal Coordinated Time (UTC).</p>
    pub fn cluster_creation_time(&self) -> ::std::option::Option<&str> {
        self.cluster_creation_time.as_deref()
    }
    /// <p>The status of the Elastic DocumentDB snapshot.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::Status> {
        self.status.as_ref()
    }
    /// <p>A list of the IDs of the VPC security groups associated with the cluster snapshot.</p>
    pub fn vpc_security_group_ids(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.vpc_security_group_ids.as_deref()
    }
    /// <p>The name of the Elastic DocumentDB cluster administrator.</p>
    pub fn admin_user_name(&self) -> ::std::option::Option<&str> {
        self.admin_user_name.as_deref()
    }
    /// <p>The KMS key identifier to use to encrypt the Elastic DocumentDB cluster.</p>
    pub fn kms_key_id(&self) -> ::std::option::Option<&str> {
        self.kms_key_id.as_deref()
    }
}
impl ClusterSnapshot {
    /// Creates a new builder-style object to manufacture [`ClusterSnapshot`](crate::types::ClusterSnapshot).
    pub fn builder() -> crate::types::builders::ClusterSnapshotBuilder {
        crate::types::builders::ClusterSnapshotBuilder::default()
    }
}

/// A builder for [`ClusterSnapshot`](crate::types::ClusterSnapshot).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ClusterSnapshotBuilder {
    pub(crate) subnet_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) snapshot_name: ::std::option::Option<::std::string::String>,
    pub(crate) snapshot_arn: ::std::option::Option<::std::string::String>,
    pub(crate) snapshot_creation_time: ::std::option::Option<::std::string::String>,
    pub(crate) cluster_arn: ::std::option::Option<::std::string::String>,
    pub(crate) cluster_creation_time: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::Status>,
    pub(crate) vpc_security_group_ids:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) admin_user_name: ::std::option::Option<::std::string::String>,
    pub(crate) kms_key_id: ::std::option::Option<::std::string::String>,
}
impl ClusterSnapshotBuilder {
    /// Appends an item to `subnet_ids`.
    ///
    /// To override the contents of this collection use [`set_subnet_ids`](Self::set_subnet_ids).
    ///
    /// <p>A list of the IDs of subnets associated with the DB cluster snapshot.</p>
    pub fn subnet_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.subnet_ids.unwrap_or_default();
        v.push(input.into());
        self.subnet_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of the IDs of subnets associated with the DB cluster snapshot.</p>
    pub fn set_subnet_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.subnet_ids = input;
        self
    }
    /// <p>The name of the Elastic DocumentDB snapshot.</p>
    pub fn snapshot_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.snapshot_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the Elastic DocumentDB snapshot.</p>
    pub fn set_snapshot_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.snapshot_name = input;
        self
    }
    /// <p>The arn of the Elastic DocumentDB snapshot</p>
    pub fn snapshot_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.snapshot_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The arn of the Elastic DocumentDB snapshot</p>
    pub fn set_snapshot_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.snapshot_arn = input;
        self
    }
    /// <p>The time when the Elastic DocumentDB snapshot was created in Universal Coordinated Time (UTC).</p>
    pub fn snapshot_creation_time(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.snapshot_creation_time = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The time when the Elastic DocumentDB snapshot was created in Universal Coordinated Time (UTC).</p>
    pub fn set_snapshot_creation_time(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.snapshot_creation_time = input;
        self
    }
    /// <p>The arn of the Elastic DocumentDB cluster.</p>
    pub fn cluster_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.cluster_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The arn of the Elastic DocumentDB cluster.</p>
    pub fn set_cluster_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.cluster_arn = input;
        self
    }
    /// <p>The time when the Elastic DocumentDB cluster was created in Universal Coordinated Time (UTC).</p>
    pub fn cluster_creation_time(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.cluster_creation_time = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The time when the Elastic DocumentDB cluster was created in Universal Coordinated Time (UTC).</p>
    pub fn set_cluster_creation_time(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.cluster_creation_time = input;
        self
    }
    /// <p>The status of the Elastic DocumentDB snapshot.</p>
    pub fn status(mut self, input: crate::types::Status) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the Elastic DocumentDB snapshot.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::Status>) -> Self {
        self.status = input;
        self
    }
    /// Appends an item to `vpc_security_group_ids`.
    ///
    /// To override the contents of this collection use [`set_vpc_security_group_ids`](Self::set_vpc_security_group_ids).
    ///
    /// <p>A list of the IDs of the VPC security groups associated with the cluster snapshot.</p>
    pub fn vpc_security_group_ids(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.vpc_security_group_ids.unwrap_or_default();
        v.push(input.into());
        self.vpc_security_group_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of the IDs of the VPC security groups associated with the cluster snapshot.</p>
    pub fn set_vpc_security_group_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.vpc_security_group_ids = input;
        self
    }
    /// <p>The name of the Elastic DocumentDB cluster administrator.</p>
    pub fn admin_user_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.admin_user_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the Elastic DocumentDB cluster administrator.</p>
    pub fn set_admin_user_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.admin_user_name = input;
        self
    }
    /// <p>The KMS key identifier to use to encrypt the Elastic DocumentDB cluster.</p>
    pub fn kms_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.kms_key_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The KMS key identifier to use to encrypt the Elastic DocumentDB cluster.</p>
    pub fn set_kms_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.kms_key_id = input;
        self
    }
    /// Consumes the builder and constructs a [`ClusterSnapshot`](crate::types::ClusterSnapshot).
    pub fn build(self) -> crate::types::ClusterSnapshot {
        crate::types::ClusterSnapshot {
            subnet_ids: self.subnet_ids,
            snapshot_name: self.snapshot_name,
            snapshot_arn: self.snapshot_arn,
            snapshot_creation_time: self.snapshot_creation_time,
            cluster_arn: self.cluster_arn,
            cluster_creation_time: self.cluster_creation_time,
            status: self.status,
            vpc_security_group_ids: self.vpc_security_group_ids,
            admin_user_name: self.admin_user_name,
            kms_key_id: self.kms_key_id,
        }
    }
}
