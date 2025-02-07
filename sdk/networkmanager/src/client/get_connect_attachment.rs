// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetConnectAttachment`](crate::operation::get_connect_attachment::builders::GetConnectAttachmentFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`attachment_id(impl ::std::convert::Into<String>)`](crate::operation::get_connect_attachment::builders::GetConnectAttachmentFluentBuilder::attachment_id) / [`set_attachment_id(Option<String>)`](crate::operation::get_connect_attachment::builders::GetConnectAttachmentFluentBuilder::set_attachment_id): <p>The ID of the attachment.</p>
    /// - On success, responds with [`GetConnectAttachmentOutput`](crate::operation::get_connect_attachment::GetConnectAttachmentOutput) with field(s):
    ///   - [`connect_attachment(Option<ConnectAttachment>)`](crate::operation::get_connect_attachment::GetConnectAttachmentOutput::connect_attachment): <p>Details about the Connect attachment.</p>
    /// - On failure, responds with [`SdkError<GetConnectAttachmentError>`](crate::operation::get_connect_attachment::GetConnectAttachmentError)
    pub fn get_connect_attachment(
        &self,
    ) -> crate::operation::get_connect_attachment::builders::GetConnectAttachmentFluentBuilder {
        crate::operation::get_connect_attachment::builders::GetConnectAttachmentFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
