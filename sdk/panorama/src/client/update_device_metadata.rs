// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateDeviceMetadata`](crate::operation::update_device_metadata::builders::UpdateDeviceMetadataFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`device_id(impl ::std::convert::Into<String>)`](crate::operation::update_device_metadata::builders::UpdateDeviceMetadataFluentBuilder::device_id) / [`set_device_id(Option<String>)`](crate::operation::update_device_metadata::builders::UpdateDeviceMetadataFluentBuilder::set_device_id): <p>The device's ID.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::update_device_metadata::builders::UpdateDeviceMetadataFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_device_metadata::builders::UpdateDeviceMetadataFluentBuilder::set_description): <p>A description for the device.</p>
    /// - On success, responds with [`UpdateDeviceMetadataOutput`](crate::operation::update_device_metadata::UpdateDeviceMetadataOutput) with field(s):
    ///   - [`device_id(Option<String>)`](crate::operation::update_device_metadata::UpdateDeviceMetadataOutput::device_id): <p>The device's ID.</p>
    /// - On failure, responds with [`SdkError<UpdateDeviceMetadataError>`](crate::operation::update_device_metadata::UpdateDeviceMetadataError)
    pub fn update_device_metadata(
        &self,
    ) -> crate::operation::update_device_metadata::builders::UpdateDeviceMetadataFluentBuilder {
        crate::operation::update_device_metadata::builders::UpdateDeviceMetadataFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
