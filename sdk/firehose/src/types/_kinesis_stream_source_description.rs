// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Details about a Kinesis data stream used as the source for a Kinesis Data Firehose delivery stream.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct KinesisStreamSourceDescription {
    /// <p>The Amazon Resource Name (ARN) of the source Kinesis data stream. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-kinesis-streams">Amazon Kinesis Data Streams ARN Format</a>.</p>
    #[doc(hidden)]
    pub kinesis_stream_arn: ::std::option::Option<::std::string::String>,
    /// <p>The ARN of the role used by the source Kinesis data stream. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-iam">Amazon Web Services Identity and Access Management (IAM) ARN Format</a>.</p>
    #[doc(hidden)]
    pub role_arn: ::std::option::Option<::std::string::String>,
    /// <p>Kinesis Data Firehose starts retrieving records from the Kinesis data stream starting with this timestamp.</p>
    #[doc(hidden)]
    pub delivery_start_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl KinesisStreamSourceDescription {
    /// <p>The Amazon Resource Name (ARN) of the source Kinesis data stream. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-kinesis-streams">Amazon Kinesis Data Streams ARN Format</a>.</p>
    pub fn kinesis_stream_arn(&self) -> ::std::option::Option<&str> {
        self.kinesis_stream_arn.as_deref()
    }
    /// <p>The ARN of the role used by the source Kinesis data stream. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-iam">Amazon Web Services Identity and Access Management (IAM) ARN Format</a>.</p>
    pub fn role_arn(&self) -> ::std::option::Option<&str> {
        self.role_arn.as_deref()
    }
    /// <p>Kinesis Data Firehose starts retrieving records from the Kinesis data stream starting with this timestamp.</p>
    pub fn delivery_start_timestamp(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.delivery_start_timestamp.as_ref()
    }
}
impl KinesisStreamSourceDescription {
    /// Creates a new builder-style object to manufacture [`KinesisStreamSourceDescription`](crate::types::KinesisStreamSourceDescription).
    pub fn builder() -> crate::types::builders::KinesisStreamSourceDescriptionBuilder {
        crate::types::builders::KinesisStreamSourceDescriptionBuilder::default()
    }
}

/// A builder for [`KinesisStreamSourceDescription`](crate::types::KinesisStreamSourceDescription).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct KinesisStreamSourceDescriptionBuilder {
    pub(crate) kinesis_stream_arn: ::std::option::Option<::std::string::String>,
    pub(crate) role_arn: ::std::option::Option<::std::string::String>,
    pub(crate) delivery_start_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl KinesisStreamSourceDescriptionBuilder {
    /// <p>The Amazon Resource Name (ARN) of the source Kinesis data stream. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-kinesis-streams">Amazon Kinesis Data Streams ARN Format</a>.</p>
    pub fn kinesis_stream_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.kinesis_stream_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the source Kinesis data stream. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-kinesis-streams">Amazon Kinesis Data Streams ARN Format</a>.</p>
    pub fn set_kinesis_stream_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.kinesis_stream_arn = input;
        self
    }
    /// <p>The ARN of the role used by the source Kinesis data stream. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-iam">Amazon Web Services Identity and Access Management (IAM) ARN Format</a>.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.role_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the role used by the source Kinesis data stream. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-iam">Amazon Web Services Identity and Access Management (IAM) ARN Format</a>.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.role_arn = input;
        self
    }
    /// <p>Kinesis Data Firehose starts retrieving records from the Kinesis data stream starting with this timestamp.</p>
    pub fn delivery_start_timestamp(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.delivery_start_timestamp = ::std::option::Option::Some(input);
        self
    }
    /// <p>Kinesis Data Firehose starts retrieving records from the Kinesis data stream starting with this timestamp.</p>
    pub fn set_delivery_start_timestamp(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.delivery_start_timestamp = input;
        self
    }
    /// Consumes the builder and constructs a [`KinesisStreamSourceDescription`](crate::types::KinesisStreamSourceDescription).
    pub fn build(self) -> crate::types::KinesisStreamSourceDescription {
        crate::types::KinesisStreamSourceDescription {
            kinesis_stream_arn: self.kinesis_stream_arn,
            role_arn: self.role_arn,
            delivery_start_timestamp: self.delivery_start_timestamp,
        }
    }
}
