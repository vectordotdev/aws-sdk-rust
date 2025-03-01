// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the parameters for the connector, as identified by the <code>ConnectorId</code>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribedConnector {
    /// <p>The unique Amazon Resource Name (ARN) for the connector.</p>
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>The unique identifier for the connector.</p>
    #[doc(hidden)]
    pub connector_id: ::std::option::Option<::std::string::String>,
    /// <p>The URL of the partner's AS2 endpoint.</p>
    #[doc(hidden)]
    pub url: ::std::option::Option<::std::string::String>,
    /// <p>A structure that contains the parameters for a connector object.</p>
    #[doc(hidden)]
    pub as2_config: ::std::option::Option<crate::types::As2ConnectorConfig>,
    /// <p>With AS2, you can send files by calling <code>StartFileTransfer</code> and specifying the file paths in the request parameter, <code>SendFilePaths</code>. We use the file’s parent directory (for example, for <code>--send-file-paths /bucket/dir/file.txt</code>, parent directory is <code>/bucket/dir/</code>) to temporarily store a processed AS2 message file, store the MDN when we receive them from the partner, and write a final JSON file containing relevant metadata of the transmission. So, the <code>AccessRole</code> needs to provide read and write access to the parent directory of the file location used in the <code>StartFileTransfer</code> request. Additionally, you need to provide read and write access to the parent directory of the files that you intend to send with <code>StartFileTransfer</code>.</p>
    #[doc(hidden)]
    pub access_role: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the Identity and Access Management (IAM) role that allows a connector to turn on CloudWatch logging for Amazon S3 events. When set, you can view connector activity in your CloudWatch logs.</p>
    #[doc(hidden)]
    pub logging_role: ::std::option::Option<::std::string::String>,
    /// <p>Key-value pairs that can be used to group and search for connectors.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl DescribedConnector {
    /// <p>The unique Amazon Resource Name (ARN) for the connector.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>The unique identifier for the connector.</p>
    pub fn connector_id(&self) -> ::std::option::Option<&str> {
        self.connector_id.as_deref()
    }
    /// <p>The URL of the partner's AS2 endpoint.</p>
    pub fn url(&self) -> ::std::option::Option<&str> {
        self.url.as_deref()
    }
    /// <p>A structure that contains the parameters for a connector object.</p>
    pub fn as2_config(&self) -> ::std::option::Option<&crate::types::As2ConnectorConfig> {
        self.as2_config.as_ref()
    }
    /// <p>With AS2, you can send files by calling <code>StartFileTransfer</code> and specifying the file paths in the request parameter, <code>SendFilePaths</code>. We use the file’s parent directory (for example, for <code>--send-file-paths /bucket/dir/file.txt</code>, parent directory is <code>/bucket/dir/</code>) to temporarily store a processed AS2 message file, store the MDN when we receive them from the partner, and write a final JSON file containing relevant metadata of the transmission. So, the <code>AccessRole</code> needs to provide read and write access to the parent directory of the file location used in the <code>StartFileTransfer</code> request. Additionally, you need to provide read and write access to the parent directory of the files that you intend to send with <code>StartFileTransfer</code>.</p>
    pub fn access_role(&self) -> ::std::option::Option<&str> {
        self.access_role.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the Identity and Access Management (IAM) role that allows a connector to turn on CloudWatch logging for Amazon S3 events. When set, you can view connector activity in your CloudWatch logs.</p>
    pub fn logging_role(&self) -> ::std::option::Option<&str> {
        self.logging_role.as_deref()
    }
    /// <p>Key-value pairs that can be used to group and search for connectors.</p>
    pub fn tags(&self) -> ::std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
}
impl DescribedConnector {
    /// Creates a new builder-style object to manufacture [`DescribedConnector`](crate::types::DescribedConnector).
    pub fn builder() -> crate::types::builders::DescribedConnectorBuilder {
        crate::types::builders::DescribedConnectorBuilder::default()
    }
}

/// A builder for [`DescribedConnector`](crate::types::DescribedConnector).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribedConnectorBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) connector_id: ::std::option::Option<::std::string::String>,
    pub(crate) url: ::std::option::Option<::std::string::String>,
    pub(crate) as2_config: ::std::option::Option<crate::types::As2ConnectorConfig>,
    pub(crate) access_role: ::std::option::Option<::std::string::String>,
    pub(crate) logging_role: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl DescribedConnectorBuilder {
    /// <p>The unique Amazon Resource Name (ARN) for the connector.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique Amazon Resource Name (ARN) for the connector.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The unique identifier for the connector.</p>
    pub fn connector_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.connector_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier for the connector.</p>
    pub fn set_connector_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.connector_id = input;
        self
    }
    /// <p>The URL of the partner's AS2 endpoint.</p>
    pub fn url(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.url = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The URL of the partner's AS2 endpoint.</p>
    pub fn set_url(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.url = input;
        self
    }
    /// <p>A structure that contains the parameters for a connector object.</p>
    pub fn as2_config(mut self, input: crate::types::As2ConnectorConfig) -> Self {
        self.as2_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>A structure that contains the parameters for a connector object.</p>
    pub fn set_as2_config(
        mut self,
        input: ::std::option::Option<crate::types::As2ConnectorConfig>,
    ) -> Self {
        self.as2_config = input;
        self
    }
    /// <p>With AS2, you can send files by calling <code>StartFileTransfer</code> and specifying the file paths in the request parameter, <code>SendFilePaths</code>. We use the file’s parent directory (for example, for <code>--send-file-paths /bucket/dir/file.txt</code>, parent directory is <code>/bucket/dir/</code>) to temporarily store a processed AS2 message file, store the MDN when we receive them from the partner, and write a final JSON file containing relevant metadata of the transmission. So, the <code>AccessRole</code> needs to provide read and write access to the parent directory of the file location used in the <code>StartFileTransfer</code> request. Additionally, you need to provide read and write access to the parent directory of the files that you intend to send with <code>StartFileTransfer</code>.</p>
    pub fn access_role(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.access_role = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>With AS2, you can send files by calling <code>StartFileTransfer</code> and specifying the file paths in the request parameter, <code>SendFilePaths</code>. We use the file’s parent directory (for example, for <code>--send-file-paths /bucket/dir/file.txt</code>, parent directory is <code>/bucket/dir/</code>) to temporarily store a processed AS2 message file, store the MDN when we receive them from the partner, and write a final JSON file containing relevant metadata of the transmission. So, the <code>AccessRole</code> needs to provide read and write access to the parent directory of the file location used in the <code>StartFileTransfer</code> request. Additionally, you need to provide read and write access to the parent directory of the files that you intend to send with <code>StartFileTransfer</code>.</p>
    pub fn set_access_role(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.access_role = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Identity and Access Management (IAM) role that allows a connector to turn on CloudWatch logging for Amazon S3 events. When set, you can view connector activity in your CloudWatch logs.</p>
    pub fn logging_role(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.logging_role = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Identity and Access Management (IAM) role that allows a connector to turn on CloudWatch logging for Amazon S3 events. When set, you can view connector activity in your CloudWatch logs.</p>
    pub fn set_logging_role(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.logging_role = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Key-value pairs that can be used to group and search for connectors.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>Key-value pairs that can be used to group and search for connectors.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribedConnector`](crate::types::DescribedConnector).
    pub fn build(self) -> crate::types::DescribedConnector {
        crate::types::DescribedConnector {
            arn: self.arn,
            connector_id: self.connector_id,
            url: self.url,
            as2_config: self.as2_config,
            access_role: self.access_role,
            logging_role: self.logging_role,
            tags: self.tags,
        }
    }
}
