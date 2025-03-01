// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The snapshot configuration to use when creating an OpenZFS volume from a snapshot. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateOpenZfsOriginSnapshotConfiguration {
    /// <p>The Amazon Resource Name (ARN) for a given resource. ARNs uniquely identify Amazon Web Services resources. We require an ARN when you need to specify a resource unambiguously across all of Amazon Web Services. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>.</p>
    #[doc(hidden)]
    pub snapshot_arn: ::std::option::Option<::std::string::String>,
    /// <p>The strategy used when copying data from the snapshot to the new volume. </p>
    /// <ul>
    /// <li> <p> <code>CLONE</code> - The new volume references the data in the origin snapshot. Cloning a snapshot is faster than copying data from the snapshot to a new volume and doesn't consume disk throughput. However, the origin snapshot can't be deleted if there is a volume using its copied data. </p> </li>
    /// <li> <p> <code>FULL_COPY</code> - Copies all data from the snapshot to the new volume. </p> </li>
    /// </ul>
    #[doc(hidden)]
    pub copy_strategy: ::std::option::Option<crate::types::OpenZfsCopyStrategy>,
}
impl CreateOpenZfsOriginSnapshotConfiguration {
    /// <p>The Amazon Resource Name (ARN) for a given resource. ARNs uniquely identify Amazon Web Services resources. We require an ARN when you need to specify a resource unambiguously across all of Amazon Web Services. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>.</p>
    pub fn snapshot_arn(&self) -> ::std::option::Option<&str> {
        self.snapshot_arn.as_deref()
    }
    /// <p>The strategy used when copying data from the snapshot to the new volume. </p>
    /// <ul>
    /// <li> <p> <code>CLONE</code> - The new volume references the data in the origin snapshot. Cloning a snapshot is faster than copying data from the snapshot to a new volume and doesn't consume disk throughput. However, the origin snapshot can't be deleted if there is a volume using its copied data. </p> </li>
    /// <li> <p> <code>FULL_COPY</code> - Copies all data from the snapshot to the new volume. </p> </li>
    /// </ul>
    pub fn copy_strategy(&self) -> ::std::option::Option<&crate::types::OpenZfsCopyStrategy> {
        self.copy_strategy.as_ref()
    }
}
impl CreateOpenZfsOriginSnapshotConfiguration {
    /// Creates a new builder-style object to manufacture [`CreateOpenZfsOriginSnapshotConfiguration`](crate::types::CreateOpenZfsOriginSnapshotConfiguration).
    pub fn builder() -> crate::types::builders::CreateOpenZfsOriginSnapshotConfigurationBuilder {
        crate::types::builders::CreateOpenZfsOriginSnapshotConfigurationBuilder::default()
    }
}

/// A builder for [`CreateOpenZfsOriginSnapshotConfiguration`](crate::types::CreateOpenZfsOriginSnapshotConfiguration).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateOpenZfsOriginSnapshotConfigurationBuilder {
    pub(crate) snapshot_arn: ::std::option::Option<::std::string::String>,
    pub(crate) copy_strategy: ::std::option::Option<crate::types::OpenZfsCopyStrategy>,
}
impl CreateOpenZfsOriginSnapshotConfigurationBuilder {
    /// <p>The Amazon Resource Name (ARN) for a given resource. ARNs uniquely identify Amazon Web Services resources. We require an ARN when you need to specify a resource unambiguously across all of Amazon Web Services. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>.</p>
    pub fn snapshot_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.snapshot_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) for a given resource. ARNs uniquely identify Amazon Web Services resources. We require an ARN when you need to specify a resource unambiguously across all of Amazon Web Services. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>.</p>
    pub fn set_snapshot_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.snapshot_arn = input;
        self
    }
    /// <p>The strategy used when copying data from the snapshot to the new volume. </p>
    /// <ul>
    /// <li> <p> <code>CLONE</code> - The new volume references the data in the origin snapshot. Cloning a snapshot is faster than copying data from the snapshot to a new volume and doesn't consume disk throughput. However, the origin snapshot can't be deleted if there is a volume using its copied data. </p> </li>
    /// <li> <p> <code>FULL_COPY</code> - Copies all data from the snapshot to the new volume. </p> </li>
    /// </ul>
    pub fn copy_strategy(mut self, input: crate::types::OpenZfsCopyStrategy) -> Self {
        self.copy_strategy = ::std::option::Option::Some(input);
        self
    }
    /// <p>The strategy used when copying data from the snapshot to the new volume. </p>
    /// <ul>
    /// <li> <p> <code>CLONE</code> - The new volume references the data in the origin snapshot. Cloning a snapshot is faster than copying data from the snapshot to a new volume and doesn't consume disk throughput. However, the origin snapshot can't be deleted if there is a volume using its copied data. </p> </li>
    /// <li> <p> <code>FULL_COPY</code> - Copies all data from the snapshot to the new volume. </p> </li>
    /// </ul>
    pub fn set_copy_strategy(
        mut self,
        input: ::std::option::Option<crate::types::OpenZfsCopyStrategy>,
    ) -> Self {
        self.copy_strategy = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateOpenZfsOriginSnapshotConfiguration`](crate::types::CreateOpenZfsOriginSnapshotConfiguration).
    pub fn build(self) -> crate::types::CreateOpenZfsOriginSnapshotConfiguration {
        crate::types::CreateOpenZfsOriginSnapshotConfiguration {
            snapshot_arn: self.snapshot_arn,
            copy_strategy: self.copy_strategy,
        }
    }
}
