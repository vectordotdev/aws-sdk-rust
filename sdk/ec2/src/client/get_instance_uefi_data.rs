// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetInstanceUefiData`](crate::operation::get_instance_uefi_data::builders::GetInstanceUefiDataFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_id(impl ::std::convert::Into<String>)`](crate::operation::get_instance_uefi_data::builders::GetInstanceUefiDataFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::get_instance_uefi_data::builders::GetInstanceUefiDataFluentBuilder::set_instance_id): <p>The ID of the instance from which to retrieve the UEFI data.</p>
    ///   - [`dry_run(bool)`](crate::operation::get_instance_uefi_data::builders::GetInstanceUefiDataFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::get_instance_uefi_data::builders::GetInstanceUefiDataFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`GetInstanceUefiDataOutput`](crate::operation::get_instance_uefi_data::GetInstanceUefiDataOutput) with field(s):
    ///   - [`instance_id(Option<String>)`](crate::operation::get_instance_uefi_data::GetInstanceUefiDataOutput::instance_id): <p>The ID of the instance from which to retrieve the UEFI data.</p>
    ///   - [`uefi_data(Option<String>)`](crate::operation::get_instance_uefi_data::GetInstanceUefiDataOutput::uefi_data): <p>Base64 representation of the non-volatile UEFI variable store.</p>
    /// - On failure, responds with [`SdkError<GetInstanceUefiDataError>`](crate::operation::get_instance_uefi_data::GetInstanceUefiDataError)
    pub fn get_instance_uefi_data(
        &self,
    ) -> crate::operation::get_instance_uefi_data::builders::GetInstanceUefiDataFluentBuilder {
        crate::operation::get_instance_uefi_data::builders::GetInstanceUefiDataFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
