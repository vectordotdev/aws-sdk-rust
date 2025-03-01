// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetBucketRequestPayment`](crate::operation::get_bucket_request_payment::builders::GetBucketRequestPaymentFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bucket(impl ::std::convert::Into<String>)`](crate::operation::get_bucket_request_payment::builders::GetBucketRequestPaymentFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::get_bucket_request_payment::builders::GetBucketRequestPaymentFluentBuilder::set_bucket): <p>The name of the bucket for which to get the payment request configuration</p>
    ///   - [`expected_bucket_owner(impl ::std::convert::Into<String>)`](crate::operation::get_bucket_request_payment::builders::GetBucketRequestPaymentFluentBuilder::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::operation::get_bucket_request_payment::builders::GetBucketRequestPaymentFluentBuilder::set_expected_bucket_owner): <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    /// - On success, responds with [`GetBucketRequestPaymentOutput`](crate::operation::get_bucket_request_payment::GetBucketRequestPaymentOutput) with field(s):
    ///   - [`payer(Option<Payer>)`](crate::operation::get_bucket_request_payment::GetBucketRequestPaymentOutput::payer): <p>Specifies who pays for the download and request fees.</p>
    /// - On failure, responds with [`SdkError<GetBucketRequestPaymentError>`](crate::operation::get_bucket_request_payment::GetBucketRequestPaymentError)
    pub fn get_bucket_request_payment(
        &self,
    ) -> crate::operation::get_bucket_request_payment::builders::GetBucketRequestPaymentFluentBuilder
    {
        crate::operation::get_bucket_request_payment::builders::GetBucketRequestPaymentFluentBuilder::new(self.handle.clone())
    }
}
