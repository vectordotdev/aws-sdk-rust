// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the raw data of the message.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RawMessage {
    /// <p>The raw data of the message. This data needs to base64-encoded if you are accessing Amazon SES directly through the HTTPS interface. If you are accessing Amazon SES using an AWS SDK, the SDK takes care of the base 64-encoding for you. In all cases, the client must ensure that the message format complies with Internet email standards regarding email header fields, MIME types, and MIME encoding.</p>
    /// <p>The To:, CC:, and BCC: headers in the raw message can contain a group list.</p>
    /// <p>If you are using <code>SendRawEmail</code> with sending authorization, you can include X-headers in the raw message to specify the "Source," "From," and "Return-Path" addresses. For more information, see the documentation for <code>SendRawEmail</code>. </p> <important>
    /// <p>Do not include these X-headers in the DKIM signature, because they are removed by Amazon SES before sending the email.</p>
    /// </important>
    /// <p>For more information, go to the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-email-raw.html">Amazon SES Developer Guide</a>.</p>
    #[doc(hidden)]
    pub data: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl RawMessage {
    /// <p>The raw data of the message. This data needs to base64-encoded if you are accessing Amazon SES directly through the HTTPS interface. If you are accessing Amazon SES using an AWS SDK, the SDK takes care of the base 64-encoding for you. In all cases, the client must ensure that the message format complies with Internet email standards regarding email header fields, MIME types, and MIME encoding.</p>
    /// <p>The To:, CC:, and BCC: headers in the raw message can contain a group list.</p>
    /// <p>If you are using <code>SendRawEmail</code> with sending authorization, you can include X-headers in the raw message to specify the "Source," "From," and "Return-Path" addresses. For more information, see the documentation for <code>SendRawEmail</code>. </p> <important>
    /// <p>Do not include these X-headers in the DKIM signature, because they are removed by Amazon SES before sending the email.</p>
    /// </important>
    /// <p>For more information, go to the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-email-raw.html">Amazon SES Developer Guide</a>.</p>
    pub fn data(&self) -> ::std::option::Option<&::aws_smithy_types::Blob> {
        self.data.as_ref()
    }
}
impl RawMessage {
    /// Creates a new builder-style object to manufacture [`RawMessage`](crate::types::RawMessage).
    pub fn builder() -> crate::types::builders::RawMessageBuilder {
        crate::types::builders::RawMessageBuilder::default()
    }
}

/// A builder for [`RawMessage`](crate::types::RawMessage).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RawMessageBuilder {
    pub(crate) data: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl RawMessageBuilder {
    /// <p>The raw data of the message. This data needs to base64-encoded if you are accessing Amazon SES directly through the HTTPS interface. If you are accessing Amazon SES using an AWS SDK, the SDK takes care of the base 64-encoding for you. In all cases, the client must ensure that the message format complies with Internet email standards regarding email header fields, MIME types, and MIME encoding.</p>
    /// <p>The To:, CC:, and BCC: headers in the raw message can contain a group list.</p>
    /// <p>If you are using <code>SendRawEmail</code> with sending authorization, you can include X-headers in the raw message to specify the "Source," "From," and "Return-Path" addresses. For more information, see the documentation for <code>SendRawEmail</code>. </p> <important>
    /// <p>Do not include these X-headers in the DKIM signature, because they are removed by Amazon SES before sending the email.</p>
    /// </important>
    /// <p>For more information, go to the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-email-raw.html">Amazon SES Developer Guide</a>.</p>
    pub fn data(mut self, input: ::aws_smithy_types::Blob) -> Self {
        self.data = ::std::option::Option::Some(input);
        self
    }
    /// <p>The raw data of the message. This data needs to base64-encoded if you are accessing Amazon SES directly through the HTTPS interface. If you are accessing Amazon SES using an AWS SDK, the SDK takes care of the base 64-encoding for you. In all cases, the client must ensure that the message format complies with Internet email standards regarding email header fields, MIME types, and MIME encoding.</p>
    /// <p>The To:, CC:, and BCC: headers in the raw message can contain a group list.</p>
    /// <p>If you are using <code>SendRawEmail</code> with sending authorization, you can include X-headers in the raw message to specify the "Source," "From," and "Return-Path" addresses. For more information, see the documentation for <code>SendRawEmail</code>. </p> <important>
    /// <p>Do not include these X-headers in the DKIM signature, because they are removed by Amazon SES before sending the email.</p>
    /// </important>
    /// <p>For more information, go to the <a href="https://docs.aws.amazon.com/ses/latest/DeveloperGuide/send-email-raw.html">Amazon SES Developer Guide</a>.</p>
    pub fn set_data(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.data = input;
        self
    }
    /// Consumes the builder and constructs a [`RawMessage`](crate::types::RawMessage).
    pub fn build(self) -> crate::types::RawMessage {
        crate::types::RawMessage { data: self.data }
    }
}
