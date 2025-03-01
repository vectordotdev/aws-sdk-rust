// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeLocationFsxOpenZfs`](crate::operation::describe_location_fsx_open_zfs::builders::DescribeLocationFsxOpenZfsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`location_arn(impl ::std::convert::Into<String>)`](crate::operation::describe_location_fsx_open_zfs::builders::DescribeLocationFsxOpenZfsFluentBuilder::location_arn) / [`set_location_arn(Option<String>)`](crate::operation::describe_location_fsx_open_zfs::builders::DescribeLocationFsxOpenZfsFluentBuilder::set_location_arn): <p>The Amazon Resource Name (ARN) of the FSx for OpenZFS location to describe.</p>
    /// - On success, responds with [`DescribeLocationFsxOpenZfsOutput`](crate::operation::describe_location_fsx_open_zfs::DescribeLocationFsxOpenZfsOutput) with field(s):
    ///   - [`location_arn(Option<String>)`](crate::operation::describe_location_fsx_open_zfs::DescribeLocationFsxOpenZfsOutput::location_arn): <p>The ARN of the FSx for OpenZFS location that was described.</p>
    ///   - [`location_uri(Option<String>)`](crate::operation::describe_location_fsx_open_zfs::DescribeLocationFsxOpenZfsOutput::location_uri): <p>The uniform resource identifier (URI) of the FSx for OpenZFS location that was described.</p>  <p>Example: <code>fsxz://us-west-2.fs-1234567890abcdef02/fsx/folderA/folder</code> </p>
    ///   - [`security_group_arns(Option<Vec<String>>)`](crate::operation::describe_location_fsx_open_zfs::DescribeLocationFsxOpenZfsOutput::security_group_arns): <p>The ARNs of the security groups that are configured for the FSx for OpenZFS file system.</p>
    ///   - [`protocol(Option<FsxProtocol>)`](crate::operation::describe_location_fsx_open_zfs::DescribeLocationFsxOpenZfsOutput::protocol): <p>The type of protocol that DataSync uses to access your file system.</p>
    ///   - [`creation_time(Option<DateTime>)`](crate::operation::describe_location_fsx_open_zfs::DescribeLocationFsxOpenZfsOutput::creation_time): <p>The time that the FSx for OpenZFS location was created.</p>
    /// - On failure, responds with [`SdkError<DescribeLocationFsxOpenZfsError>`](crate::operation::describe_location_fsx_open_zfs::DescribeLocationFsxOpenZfsError)
    pub fn describe_location_fsx_open_zfs(&self) -> crate::operation::describe_location_fsx_open_zfs::builders::DescribeLocationFsxOpenZfsFluentBuilder{
        crate::operation::describe_location_fsx_open_zfs::builders::DescribeLocationFsxOpenZfsFluentBuilder::new(self.handle.clone())
    }
}
