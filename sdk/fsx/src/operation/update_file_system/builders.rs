// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_file_system::_update_file_system_output::UpdateFileSystemOutputBuilder;

pub use crate::operation::update_file_system::_update_file_system_input::UpdateFileSystemInputBuilder;

/// Fluent builder constructing a request to `UpdateFileSystem`.
///
/// <p>Use this operation to update the configuration of an existing Amazon FSx file system. You can update multiple properties in a single request.</p>
/// <p>For FSx for Windows File Server file systems, you can update the following properties:</p>
/// <ul>
/// <li> <p> <code>AuditLogConfiguration</code> </p> </li>
/// <li> <p> <code>AutomaticBackupRetentionDays</code> </p> </li>
/// <li> <p> <code>DailyAutomaticBackupStartTime</code> </p> </li>
/// <li> <p> <code>SelfManagedActiveDirectoryConfiguration</code> </p> </li>
/// <li> <p> <code>StorageCapacity</code> </p> </li>
/// <li> <p> <code>ThroughputCapacity</code> </p> </li>
/// <li> <p> <code>WeeklyMaintenanceStartTime</code> </p> </li>
/// </ul>
/// <p>For FSx for Lustre file systems, you can update the following properties:</p>
/// <ul>
/// <li> <p> <code>AutoImportPolicy</code> </p> </li>
/// <li> <p> <code>AutomaticBackupRetentionDays</code> </p> </li>
/// <li> <p> <code>DailyAutomaticBackupStartTime</code> </p> </li>
/// <li> <p> <code>DataCompressionType</code> </p> </li>
/// <li> <p> <code>LustreRootSquashConfiguration</code> </p> </li>
/// <li> <p> <code>StorageCapacity</code> </p> </li>
/// <li> <p> <code>WeeklyMaintenanceStartTime</code> </p> </li>
/// </ul>
/// <p>For FSx for ONTAP file systems, you can update the following properties:</p>
/// <ul>
/// <li> <p> <code>AddRouteTableIds</code> </p> </li>
/// <li> <p> <code>AutomaticBackupRetentionDays</code> </p> </li>
/// <li> <p> <code>DailyAutomaticBackupStartTime</code> </p> </li>
/// <li> <p> <code>DiskIopsConfiguration</code> </p> </li>
/// <li> <p> <code>FsxAdminPassword</code> </p> </li>
/// <li> <p> <code>RemoveRouteTableIds</code> </p> </li>
/// <li> <p> <code>StorageCapacity</code> </p> </li>
/// <li> <p> <code>ThroughputCapacity</code> </p> </li>
/// <li> <p> <code>WeeklyMaintenanceStartTime</code> </p> </li>
/// </ul>
/// <p>For FSx for OpenZFS file systems, you can update the following properties:</p>
/// <ul>
/// <li> <p> <code>AutomaticBackupRetentionDays</code> </p> </li>
/// <li> <p> <code>CopyTagsToBackups</code> </p> </li>
/// <li> <p> <code>CopyTagsToVolumes</code> </p> </li>
/// <li> <p> <code>DailyAutomaticBackupStartTime</code> </p> </li>
/// <li> <p> <code>DiskIopsConfiguration</code> </p> </li>
/// <li> <p> <code>StorageCapacity</code> </p> </li>
/// <li> <p> <code>ThroughputCapacity</code> </p> </li>
/// <li> <p> <code>WeeklyMaintenanceStartTime</code> </p> </li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateFileSystemFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_file_system::builders::UpdateFileSystemInputBuilder,
}
impl UpdateFileSystemFluentBuilder {
    /// Creates a new `UpdateFileSystem`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn customize_middleware(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::update_file_system::UpdateFileSystem,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_file_system::UpdateFileSystemError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        ::std::result::Result::Ok(crate::client::customize::CustomizableOperation {
            handle,
            operation,
        })
    }

    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn send_middleware(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_file_system::UpdateFileSystemOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_file_system::UpdateFileSystemError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_file_system::UpdateFileSystemOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_file_system::UpdateFileSystemError,
        >,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::update_file_system::UpdateFileSystem,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_file_system::UpdateFileSystemError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The ID of the file system that you are updating.</p>
    pub fn file_system_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.file_system_id(input.into());
        self
    }
    /// <p>The ID of the file system that you are updating.</p>
    pub fn set_file_system_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_file_system_id(input);
        self
    }
    /// <p>A string of up to 63 ASCII characters that Amazon FSx uses to ensure idempotent updates. This string is automatically filled on your behalf when you use the Command Line Interface (CLI) or an Amazon Web Services SDK.</p>
    pub fn client_request_token(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.client_request_token(input.into());
        self
    }
    /// <p>A string of up to 63 ASCII characters that Amazon FSx uses to ensure idempotent updates. This string is automatically filled on your behalf when you use the Command Line Interface (CLI) or an Amazon Web Services SDK.</p>
    pub fn set_client_request_token(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_client_request_token(input);
        self
    }
    /// <p>Use this parameter to increase the storage capacity of an FSx for Windows File Server, FSx for Lustre, FSx for OpenZFS, or FSx for ONTAP file system. Specifies the storage capacity target value, in GiB, to increase the storage capacity for the file system that you're updating. </p> <note>
    /// <p>You can't make a storage capacity increase request if there is an existing storage capacity increase request in progress.</p>
    /// </note>
    /// <p>For Lustre file systems, the storage capacity target value can be the following:</p>
    /// <ul>
    /// <li> <p>For <code>SCRATCH_2</code>, <code>PERSISTENT_1</code>, and <code>PERSISTENT_2 SSD</code> deployment types, valid values are in multiples of 2400 GiB. The value must be greater than the current storage capacity.</p> </li>
    /// <li> <p>For <code>PERSISTENT HDD</code> file systems, valid values are multiples of 6000 GiB for 12-MBps throughput per TiB file systems and multiples of 1800 GiB for 40-MBps throughput per TiB file systems. The values must be greater than the current storage capacity.</p> </li>
    /// <li> <p>For <code>SCRATCH_1</code> file systems, you can't increase the storage capacity.</p> </li>
    /// </ul>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/fsx/latest/LustreGuide/managing-storage-capacity.html">Managing storage and throughput capacity</a> in the <i>FSx for Lustre User Guide</i>.</p>
    /// <p>For FSx for OpenZFS file systems, the storage capacity target value must be at least 10 percent greater than the current storage capacity value. For more information, see <a href="https://docs.aws.amazon.com/fsx/latest/OpenZFSGuide/managing-storage-capacity.html">Managing storage capacity</a> in the <i>FSx for OpenZFS User Guide</i>.</p>
    /// <p>For Windows file systems, the storage capacity target value must be at least 10 percent greater than the current storage capacity value. To increase storage capacity, the file system must have at least 16 MBps of throughput capacity. For more information, see <a href="https://docs.aws.amazon.com/fsx/latest/WindowsGuide/managing-storage-capacity.html">Managing storage capacity</a> in the <i>Amazon FSx for Windows File Server User Guide</i>.</p>
    /// <p>For ONTAP file systems, the storage capacity target value must be at least 10 percent greater than the current storage capacity value. For more information, see <a href="https://docs.aws.amazon.com/fsx/latest/ONTAPGuide/managing-storage-capacity.html">Managing storage capacity and provisioned IOPS</a> in the <i>Amazon FSx for NetApp ONTAP User Guide</i>.</p>
    pub fn storage_capacity(mut self, input: i32) -> Self {
        self.inner = self.inner.storage_capacity(input);
        self
    }
    /// <p>Use this parameter to increase the storage capacity of an FSx for Windows File Server, FSx for Lustre, FSx for OpenZFS, or FSx for ONTAP file system. Specifies the storage capacity target value, in GiB, to increase the storage capacity for the file system that you're updating. </p> <note>
    /// <p>You can't make a storage capacity increase request if there is an existing storage capacity increase request in progress.</p>
    /// </note>
    /// <p>For Lustre file systems, the storage capacity target value can be the following:</p>
    /// <ul>
    /// <li> <p>For <code>SCRATCH_2</code>, <code>PERSISTENT_1</code>, and <code>PERSISTENT_2 SSD</code> deployment types, valid values are in multiples of 2400 GiB. The value must be greater than the current storage capacity.</p> </li>
    /// <li> <p>For <code>PERSISTENT HDD</code> file systems, valid values are multiples of 6000 GiB for 12-MBps throughput per TiB file systems and multiples of 1800 GiB for 40-MBps throughput per TiB file systems. The values must be greater than the current storage capacity.</p> </li>
    /// <li> <p>For <code>SCRATCH_1</code> file systems, you can't increase the storage capacity.</p> </li>
    /// </ul>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/fsx/latest/LustreGuide/managing-storage-capacity.html">Managing storage and throughput capacity</a> in the <i>FSx for Lustre User Guide</i>.</p>
    /// <p>For FSx for OpenZFS file systems, the storage capacity target value must be at least 10 percent greater than the current storage capacity value. For more information, see <a href="https://docs.aws.amazon.com/fsx/latest/OpenZFSGuide/managing-storage-capacity.html">Managing storage capacity</a> in the <i>FSx for OpenZFS User Guide</i>.</p>
    /// <p>For Windows file systems, the storage capacity target value must be at least 10 percent greater than the current storage capacity value. To increase storage capacity, the file system must have at least 16 MBps of throughput capacity. For more information, see <a href="https://docs.aws.amazon.com/fsx/latest/WindowsGuide/managing-storage-capacity.html">Managing storage capacity</a> in the <i>Amazon FSx for Windows File Server User Guide</i>.</p>
    /// <p>For ONTAP file systems, the storage capacity target value must be at least 10 percent greater than the current storage capacity value. For more information, see <a href="https://docs.aws.amazon.com/fsx/latest/ONTAPGuide/managing-storage-capacity.html">Managing storage capacity and provisioned IOPS</a> in the <i>Amazon FSx for NetApp ONTAP User Guide</i>.</p>
    pub fn set_storage_capacity(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_storage_capacity(input);
        self
    }
    /// <p>The configuration updates for an Amazon FSx for Windows File Server file system.</p>
    pub fn windows_configuration(
        mut self,
        input: crate::types::UpdateFileSystemWindowsConfiguration,
    ) -> Self {
        self.inner = self.inner.windows_configuration(input);
        self
    }
    /// <p>The configuration updates for an Amazon FSx for Windows File Server file system.</p>
    pub fn set_windows_configuration(
        mut self,
        input: ::std::option::Option<crate::types::UpdateFileSystemWindowsConfiguration>,
    ) -> Self {
        self.inner = self.inner.set_windows_configuration(input);
        self
    }
    /// <p>The configuration object for Amazon FSx for Lustre file systems used in the <code>UpdateFileSystem</code> operation.</p>
    pub fn lustre_configuration(
        mut self,
        input: crate::types::UpdateFileSystemLustreConfiguration,
    ) -> Self {
        self.inner = self.inner.lustre_configuration(input);
        self
    }
    /// <p>The configuration object for Amazon FSx for Lustre file systems used in the <code>UpdateFileSystem</code> operation.</p>
    pub fn set_lustre_configuration(
        mut self,
        input: ::std::option::Option<crate::types::UpdateFileSystemLustreConfiguration>,
    ) -> Self {
        self.inner = self.inner.set_lustre_configuration(input);
        self
    }
    /// <p>The configuration updates for an Amazon FSx for NetApp ONTAP file system.</p>
    pub fn ontap_configuration(
        mut self,
        input: crate::types::UpdateFileSystemOntapConfiguration,
    ) -> Self {
        self.inner = self.inner.ontap_configuration(input);
        self
    }
    /// <p>The configuration updates for an Amazon FSx for NetApp ONTAP file system.</p>
    pub fn set_ontap_configuration(
        mut self,
        input: ::std::option::Option<crate::types::UpdateFileSystemOntapConfiguration>,
    ) -> Self {
        self.inner = self.inner.set_ontap_configuration(input);
        self
    }
    /// <p>The configuration updates for an Amazon FSx for OpenZFS file system.</p>
    pub fn open_zfs_configuration(
        mut self,
        input: crate::types::UpdateFileSystemOpenZfsConfiguration,
    ) -> Self {
        self.inner = self.inner.open_zfs_configuration(input);
        self
    }
    /// <p>The configuration updates for an Amazon FSx for OpenZFS file system.</p>
    pub fn set_open_zfs_configuration(
        mut self,
        input: ::std::option::Option<crate::types::UpdateFileSystemOpenZfsConfiguration>,
    ) -> Self {
        self.inner = self.inner.set_open_zfs_configuration(input);
        self
    }
}
