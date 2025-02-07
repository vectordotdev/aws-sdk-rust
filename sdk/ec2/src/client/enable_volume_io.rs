// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`EnableVolumeIO`](crate::operation::enable_volume_io::builders::EnableVolumeIOFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::enable_volume_io::builders::EnableVolumeIOFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::enable_volume_io::builders::EnableVolumeIOFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`volume_id(impl ::std::convert::Into<String>)`](crate::operation::enable_volume_io::builders::EnableVolumeIOFluentBuilder::volume_id) / [`set_volume_id(Option<String>)`](crate::operation::enable_volume_io::builders::EnableVolumeIOFluentBuilder::set_volume_id): <p>The ID of the volume.</p>
    /// - On success, responds with [`EnableVolumeIoOutput`](crate::operation::enable_volume_io::EnableVolumeIoOutput)
    /// - On failure, responds with [`SdkError<EnableVolumeIOError>`](crate::operation::enable_volume_io::EnableVolumeIOError)
    pub fn enable_volume_io(
        &self,
    ) -> crate::operation::enable_volume_io::builders::EnableVolumeIOFluentBuilder {
        crate::operation::enable_volume_io::builders::EnableVolumeIOFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
