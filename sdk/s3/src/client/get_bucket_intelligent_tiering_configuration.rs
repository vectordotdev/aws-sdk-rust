// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetBucketIntelligentTieringConfiguration`](crate::operation::get_bucket_intelligent_tiering_configuration::builders::GetBucketIntelligentTieringConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bucket(impl ::std::convert::Into<String>)`](crate::operation::get_bucket_intelligent_tiering_configuration::builders::GetBucketIntelligentTieringConfigurationFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::get_bucket_intelligent_tiering_configuration::builders::GetBucketIntelligentTieringConfigurationFluentBuilder::set_bucket): <p>The name of the Amazon S3 bucket whose configuration you want to modify or retrieve.</p>
    ///   - [`id(impl ::std::convert::Into<String>)`](crate::operation::get_bucket_intelligent_tiering_configuration::builders::GetBucketIntelligentTieringConfigurationFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::get_bucket_intelligent_tiering_configuration::builders::GetBucketIntelligentTieringConfigurationFluentBuilder::set_id): <p>The ID used to identify the S3 Intelligent-Tiering configuration.</p>
    /// - On success, responds with [`GetBucketIntelligentTieringConfigurationOutput`](crate::operation::get_bucket_intelligent_tiering_configuration::GetBucketIntelligentTieringConfigurationOutput) with field(s):
    ///   - [`intelligent_tiering_configuration(Option<IntelligentTieringConfiguration>)`](crate::operation::get_bucket_intelligent_tiering_configuration::GetBucketIntelligentTieringConfigurationOutput::intelligent_tiering_configuration): <p>Container for S3 Intelligent-Tiering configuration.</p>
    /// - On failure, responds with [`SdkError<GetBucketIntelligentTieringConfigurationError>`](crate::operation::get_bucket_intelligent_tiering_configuration::GetBucketIntelligentTieringConfigurationError)
    pub fn get_bucket_intelligent_tiering_configuration(&self) -> crate::operation::get_bucket_intelligent_tiering_configuration::builders::GetBucketIntelligentTieringConfigurationFluentBuilder{
        crate::operation::get_bucket_intelligent_tiering_configuration::builders::GetBucketIntelligentTieringConfigurationFluentBuilder::new(self.handle.clone())
    }
}
