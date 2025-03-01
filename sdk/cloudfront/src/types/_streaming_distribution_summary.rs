// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A summary of the information for a CloudFront streaming distribution.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StreamingDistributionSummary {
    /// <p>The identifier for the distribution, for example, <code>EDFDVBD632BHDS5</code>.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>The ARN (Amazon Resource Name) for the streaming distribution. For example: <code>arn:aws:cloudfront::123456789012:streaming-distribution/EDFDVBD632BHDS5</code>, where <code>123456789012</code> is your Amazon Web Services account ID.</p>
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>Indicates the current status of the distribution. When the status is <code>Deployed</code>, the distribution's information is fully propagated throughout the Amazon CloudFront system.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<::std::string::String>,
    /// <p>The date and time the distribution was last modified.</p>
    #[doc(hidden)]
    pub last_modified_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The domain name corresponding to the distribution, for example, <code>d111111abcdef8.cloudfront.net</code>.</p>
    #[doc(hidden)]
    pub domain_name: ::std::option::Option<::std::string::String>,
    /// <p>A complex type that contains information about the Amazon S3 bucket from which you want CloudFront to get your media files for distribution.</p>
    #[doc(hidden)]
    pub s3_origin: ::std::option::Option<crate::types::S3Origin>,
    /// <p>A complex type that contains information about CNAMEs (alternate domain names), if any, for this streaming distribution.</p>
    #[doc(hidden)]
    pub aliases: ::std::option::Option<crate::types::Aliases>,
    /// <p>A complex type that specifies the Amazon Web Services accounts, if any, that you want to allow to create signed URLs for private content. If you want to require signed URLs in requests for objects in the target origin that match the <code>PathPattern</code> for this cache behavior, specify <code>true</code> for <code>Enabled</code>, and specify the applicable values for <code>Quantity</code> and <code>Items</code>.If you don't want to require signed URLs in requests for objects that match <code>PathPattern</code>, specify <code>false</code> for <code>Enabled</code> and <code>0</code> for <code>Quantity</code>. Omit <code>Items</code>. To add, change, or remove one or more trusted signers, change <code>Enabled</code> to <code>true</code> (if it's currently <code>false</code>), change <code>Quantity</code> as applicable, and specify all of the trusted signers that you want to include in the updated distribution.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">Serving Private Content through CloudFront</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    #[doc(hidden)]
    pub trusted_signers: ::std::option::Option<crate::types::TrustedSigners>,
    /// <p>The comment originally specified when this distribution was created.</p>
    #[doc(hidden)]
    pub comment: ::std::option::Option<::std::string::String>,
    /// <p>A complex type that contains information about price class for this streaming distribution.</p>
    #[doc(hidden)]
    pub price_class: ::std::option::Option<crate::types::PriceClass>,
    /// <p>Whether the distribution is enabled to accept end user requests for content.</p>
    #[doc(hidden)]
    pub enabled: ::std::option::Option<bool>,
}
impl StreamingDistributionSummary {
    /// <p>The identifier for the distribution, for example, <code>EDFDVBD632BHDS5</code>.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The ARN (Amazon Resource Name) for the streaming distribution. For example: <code>arn:aws:cloudfront::123456789012:streaming-distribution/EDFDVBD632BHDS5</code>, where <code>123456789012</code> is your Amazon Web Services account ID.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>Indicates the current status of the distribution. When the status is <code>Deployed</code>, the distribution's information is fully propagated throughout the Amazon CloudFront system.</p>
    pub fn status(&self) -> ::std::option::Option<&str> {
        self.status.as_deref()
    }
    /// <p>The date and time the distribution was last modified.</p>
    pub fn last_modified_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_modified_time.as_ref()
    }
    /// <p>The domain name corresponding to the distribution, for example, <code>d111111abcdef8.cloudfront.net</code>.</p>
    pub fn domain_name(&self) -> ::std::option::Option<&str> {
        self.domain_name.as_deref()
    }
    /// <p>A complex type that contains information about the Amazon S3 bucket from which you want CloudFront to get your media files for distribution.</p>
    pub fn s3_origin(&self) -> ::std::option::Option<&crate::types::S3Origin> {
        self.s3_origin.as_ref()
    }
    /// <p>A complex type that contains information about CNAMEs (alternate domain names), if any, for this streaming distribution.</p>
    pub fn aliases(&self) -> ::std::option::Option<&crate::types::Aliases> {
        self.aliases.as_ref()
    }
    /// <p>A complex type that specifies the Amazon Web Services accounts, if any, that you want to allow to create signed URLs for private content. If you want to require signed URLs in requests for objects in the target origin that match the <code>PathPattern</code> for this cache behavior, specify <code>true</code> for <code>Enabled</code>, and specify the applicable values for <code>Quantity</code> and <code>Items</code>.If you don't want to require signed URLs in requests for objects that match <code>PathPattern</code>, specify <code>false</code> for <code>Enabled</code> and <code>0</code> for <code>Quantity</code>. Omit <code>Items</code>. To add, change, or remove one or more trusted signers, change <code>Enabled</code> to <code>true</code> (if it's currently <code>false</code>), change <code>Quantity</code> as applicable, and specify all of the trusted signers that you want to include in the updated distribution.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">Serving Private Content through CloudFront</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub fn trusted_signers(&self) -> ::std::option::Option<&crate::types::TrustedSigners> {
        self.trusted_signers.as_ref()
    }
    /// <p>The comment originally specified when this distribution was created.</p>
    pub fn comment(&self) -> ::std::option::Option<&str> {
        self.comment.as_deref()
    }
    /// <p>A complex type that contains information about price class for this streaming distribution.</p>
    pub fn price_class(&self) -> ::std::option::Option<&crate::types::PriceClass> {
        self.price_class.as_ref()
    }
    /// <p>Whether the distribution is enabled to accept end user requests for content.</p>
    pub fn enabled(&self) -> ::std::option::Option<bool> {
        self.enabled
    }
}
impl StreamingDistributionSummary {
    /// Creates a new builder-style object to manufacture [`StreamingDistributionSummary`](crate::types::StreamingDistributionSummary).
    pub fn builder() -> crate::types::builders::StreamingDistributionSummaryBuilder {
        crate::types::builders::StreamingDistributionSummaryBuilder::default()
    }
}

/// A builder for [`StreamingDistributionSummary`](crate::types::StreamingDistributionSummary).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct StreamingDistributionSummaryBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<::std::string::String>,
    pub(crate) last_modified_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) domain_name: ::std::option::Option<::std::string::String>,
    pub(crate) s3_origin: ::std::option::Option<crate::types::S3Origin>,
    pub(crate) aliases: ::std::option::Option<crate::types::Aliases>,
    pub(crate) trusted_signers: ::std::option::Option<crate::types::TrustedSigners>,
    pub(crate) comment: ::std::option::Option<::std::string::String>,
    pub(crate) price_class: ::std::option::Option<crate::types::PriceClass>,
    pub(crate) enabled: ::std::option::Option<bool>,
}
impl StreamingDistributionSummaryBuilder {
    /// <p>The identifier for the distribution, for example, <code>EDFDVBD632BHDS5</code>.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier for the distribution, for example, <code>EDFDVBD632BHDS5</code>.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The ARN (Amazon Resource Name) for the streaming distribution. For example: <code>arn:aws:cloudfront::123456789012:streaming-distribution/EDFDVBD632BHDS5</code>, where <code>123456789012</code> is your Amazon Web Services account ID.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN (Amazon Resource Name) for the streaming distribution. For example: <code>arn:aws:cloudfront::123456789012:streaming-distribution/EDFDVBD632BHDS5</code>, where <code>123456789012</code> is your Amazon Web Services account ID.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>Indicates the current status of the distribution. When the status is <code>Deployed</code>, the distribution's information is fully propagated throughout the Amazon CloudFront system.</p>
    pub fn status(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.status = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Indicates the current status of the distribution. When the status is <code>Deployed</code>, the distribution's information is fully propagated throughout the Amazon CloudFront system.</p>
    pub fn set_status(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.status = input;
        self
    }
    /// <p>The date and time the distribution was last modified.</p>
    pub fn last_modified_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_modified_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time the distribution was last modified.</p>
    pub fn set_last_modified_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.last_modified_time = input;
        self
    }
    /// <p>The domain name corresponding to the distribution, for example, <code>d111111abcdef8.cloudfront.net</code>.</p>
    pub fn domain_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.domain_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The domain name corresponding to the distribution, for example, <code>d111111abcdef8.cloudfront.net</code>.</p>
    pub fn set_domain_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.domain_name = input;
        self
    }
    /// <p>A complex type that contains information about the Amazon S3 bucket from which you want CloudFront to get your media files for distribution.</p>
    pub fn s3_origin(mut self, input: crate::types::S3Origin) -> Self {
        self.s3_origin = ::std::option::Option::Some(input);
        self
    }
    /// <p>A complex type that contains information about the Amazon S3 bucket from which you want CloudFront to get your media files for distribution.</p>
    pub fn set_s3_origin(mut self, input: ::std::option::Option<crate::types::S3Origin>) -> Self {
        self.s3_origin = input;
        self
    }
    /// <p>A complex type that contains information about CNAMEs (alternate domain names), if any, for this streaming distribution.</p>
    pub fn aliases(mut self, input: crate::types::Aliases) -> Self {
        self.aliases = ::std::option::Option::Some(input);
        self
    }
    /// <p>A complex type that contains information about CNAMEs (alternate domain names), if any, for this streaming distribution.</p>
    pub fn set_aliases(mut self, input: ::std::option::Option<crate::types::Aliases>) -> Self {
        self.aliases = input;
        self
    }
    /// <p>A complex type that specifies the Amazon Web Services accounts, if any, that you want to allow to create signed URLs for private content. If you want to require signed URLs in requests for objects in the target origin that match the <code>PathPattern</code> for this cache behavior, specify <code>true</code> for <code>Enabled</code>, and specify the applicable values for <code>Quantity</code> and <code>Items</code>.If you don't want to require signed URLs in requests for objects that match <code>PathPattern</code>, specify <code>false</code> for <code>Enabled</code> and <code>0</code> for <code>Quantity</code>. Omit <code>Items</code>. To add, change, or remove one or more trusted signers, change <code>Enabled</code> to <code>true</code> (if it's currently <code>false</code>), change <code>Quantity</code> as applicable, and specify all of the trusted signers that you want to include in the updated distribution.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">Serving Private Content through CloudFront</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub fn trusted_signers(mut self, input: crate::types::TrustedSigners) -> Self {
        self.trusted_signers = ::std::option::Option::Some(input);
        self
    }
    /// <p>A complex type that specifies the Amazon Web Services accounts, if any, that you want to allow to create signed URLs for private content. If you want to require signed URLs in requests for objects in the target origin that match the <code>PathPattern</code> for this cache behavior, specify <code>true</code> for <code>Enabled</code>, and specify the applicable values for <code>Quantity</code> and <code>Items</code>.If you don't want to require signed URLs in requests for objects that match <code>PathPattern</code>, specify <code>false</code> for <code>Enabled</code> and <code>0</code> for <code>Quantity</code>. Omit <code>Items</code>. To add, change, or remove one or more trusted signers, change <code>Enabled</code> to <code>true</code> (if it's currently <code>false</code>), change <code>Quantity</code> as applicable, and specify all of the trusted signers that you want to include in the updated distribution.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/PrivateContent.html">Serving Private Content through CloudFront</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
    pub fn set_trusted_signers(
        mut self,
        input: ::std::option::Option<crate::types::TrustedSigners>,
    ) -> Self {
        self.trusted_signers = input;
        self
    }
    /// <p>The comment originally specified when this distribution was created.</p>
    pub fn comment(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.comment = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The comment originally specified when this distribution was created.</p>
    pub fn set_comment(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.comment = input;
        self
    }
    /// <p>A complex type that contains information about price class for this streaming distribution.</p>
    pub fn price_class(mut self, input: crate::types::PriceClass) -> Self {
        self.price_class = ::std::option::Option::Some(input);
        self
    }
    /// <p>A complex type that contains information about price class for this streaming distribution.</p>
    pub fn set_price_class(
        mut self,
        input: ::std::option::Option<crate::types::PriceClass>,
    ) -> Self {
        self.price_class = input;
        self
    }
    /// <p>Whether the distribution is enabled to accept end user requests for content.</p>
    pub fn enabled(mut self, input: bool) -> Self {
        self.enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>Whether the distribution is enabled to accept end user requests for content.</p>
    pub fn set_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.enabled = input;
        self
    }
    /// Consumes the builder and constructs a [`StreamingDistributionSummary`](crate::types::StreamingDistributionSummary).
    pub fn build(self) -> crate::types::StreamingDistributionSummary {
        crate::types::StreamingDistributionSummary {
            id: self.id,
            arn: self.arn,
            status: self.status,
            last_modified_time: self.last_modified_time,
            domain_name: self.domain_name,
            s3_origin: self.s3_origin,
            aliases: self.aliases,
            trusted_signers: self.trusted_signers,
            comment: self.comment,
            price_class: self.price_class,
            enabled: self.enabled,
        }
    }
}
