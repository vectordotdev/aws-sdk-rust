// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateWorker`](crate::operation::create_worker::builders::CreateWorkerFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`client_token(impl ::std::convert::Into<String>)`](crate::operation::create_worker::builders::CreateWorkerFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_worker::builders::CreateWorkerFluentBuilder::set_client_token): Token used for detecting replayed requests. Replayed requests will not be performed multiple times.
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::create_worker::builders::CreateWorkerFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_worker::builders::CreateWorkerFluentBuilder::set_name): Human friendly name of the resource.
    ///   - [`fleet(impl ::std::convert::Into<String>)`](crate::operation::create_worker::builders::CreateWorkerFluentBuilder::fleet) / [`set_fleet(Option<String>)`](crate::operation::create_worker::builders::CreateWorkerFluentBuilder::set_fleet): Full ARN of the worker fleet.
    ///   - [`additional_transient_properties(impl ::std::convert::Into<String>)`](crate::operation::create_worker::builders::CreateWorkerFluentBuilder::additional_transient_properties) / [`set_additional_transient_properties(Option<String>)`](crate::operation::create_worker::builders::CreateWorkerFluentBuilder::set_additional_transient_properties): JSON blob containing unstructured worker properties that are transient and may change during regular operation.
    ///   - [`additional_fixed_properties(impl ::std::convert::Into<String>)`](crate::operation::create_worker::builders::CreateWorkerFluentBuilder::additional_fixed_properties) / [`set_additional_fixed_properties(Option<String>)`](crate::operation::create_worker::builders::CreateWorkerFluentBuilder::set_additional_fixed_properties): JSON blob containing unstructured worker properties that are fixed and won't change during regular operation.
    ///   - [`vendor_properties(VendorProperties)`](crate::operation::create_worker::builders::CreateWorkerFluentBuilder::vendor_properties) / [`set_vendor_properties(Option<VendorProperties>)`](crate::operation::create_worker::builders::CreateWorkerFluentBuilder::set_vendor_properties): Properties of the worker that are provided by the vendor FMS.
    ///   - [`position(PositionCoordinates)`](crate::operation::create_worker::builders::CreateWorkerFluentBuilder::position) / [`set_position(Option<PositionCoordinates>)`](crate::operation::create_worker::builders::CreateWorkerFluentBuilder::set_position): Supported coordinates for worker position.
    ///   - [`orientation(Orientation)`](crate::operation::create_worker::builders::CreateWorkerFluentBuilder::orientation) / [`set_orientation(Option<Orientation>)`](crate::operation::create_worker::builders::CreateWorkerFluentBuilder::set_orientation): Worker orientation measured in units clockwise from north.
    /// - On success, responds with [`CreateWorkerOutput`](crate::operation::create_worker::CreateWorkerOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::operation::create_worker::CreateWorkerOutput::arn): Full ARN of the worker.
    ///   - [`id(Option<String>)`](crate::operation::create_worker::CreateWorkerOutput::id): Filters access by the workers identifier
    ///   - [`created_at(Option<DateTime>)`](crate::operation::create_worker::CreateWorkerOutput::created_at): Timestamp at which the resource was created.
    ///   - [`updated_at(Option<DateTime>)`](crate::operation::create_worker::CreateWorkerOutput::updated_at): Timestamp at which the resource was last updated.
    ///   - [`site(Option<String>)`](crate::operation::create_worker::CreateWorkerOutput::site): Site ARN.
    /// - On failure, responds with [`SdkError<CreateWorkerError>`](crate::operation::create_worker::CreateWorkerError)
    pub fn create_worker(
        &self,
    ) -> crate::operation::create_worker::builders::CreateWorkerFluentBuilder {
        crate::operation::create_worker::builders::CreateWorkerFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
