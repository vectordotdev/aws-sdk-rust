// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`SendDataToMulticastGroup`](crate::operation::send_data_to_multicast_group::builders::SendDataToMulticastGroupFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`id(impl ::std::convert::Into<String>)`](crate::operation::send_data_to_multicast_group::builders::SendDataToMulticastGroupFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::send_data_to_multicast_group::builders::SendDataToMulticastGroupFluentBuilder::set_id): <p>The ID of the multicast group.</p>
    ///   - [`payload_data(impl ::std::convert::Into<String>)`](crate::operation::send_data_to_multicast_group::builders::SendDataToMulticastGroupFluentBuilder::payload_data) / [`set_payload_data(Option<String>)`](crate::operation::send_data_to_multicast_group::builders::SendDataToMulticastGroupFluentBuilder::set_payload_data): <p>The binary to be sent to the end device, encoded in base64.</p>
    ///   - [`wireless_metadata(MulticastWirelessMetadata)`](crate::operation::send_data_to_multicast_group::builders::SendDataToMulticastGroupFluentBuilder::wireless_metadata) / [`set_wireless_metadata(Option<MulticastWirelessMetadata>)`](crate::operation::send_data_to_multicast_group::builders::SendDataToMulticastGroupFluentBuilder::set_wireless_metadata): <p>Wireless metadata that is to be sent to multicast group.</p>
    /// - On success, responds with [`SendDataToMulticastGroupOutput`](crate::operation::send_data_to_multicast_group::SendDataToMulticastGroupOutput) with field(s):
    ///   - [`message_id(Option<String>)`](crate::operation::send_data_to_multicast_group::SendDataToMulticastGroupOutput::message_id): <p>ID of a multicast group message.</p>
    /// - On failure, responds with [`SdkError<SendDataToMulticastGroupError>`](crate::operation::send_data_to_multicast_group::SendDataToMulticastGroupError)
    pub fn send_data_to_multicast_group(&self) -> crate::operation::send_data_to_multicast_group::builders::SendDataToMulticastGroupFluentBuilder{
        crate::operation::send_data_to_multicast_group::builders::SendDataToMulticastGroupFluentBuilder::new(self.handle.clone())
    }
}
