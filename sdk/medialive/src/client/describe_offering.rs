// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeOffering`](crate::operation::describe_offering::builders::DescribeOfferingFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`offering_id(impl ::std::convert::Into<String>)`](crate::operation::describe_offering::builders::DescribeOfferingFluentBuilder::offering_id) / [`set_offering_id(Option<String>)`](crate::operation::describe_offering::builders::DescribeOfferingFluentBuilder::set_offering_id): Unique offering ID, e.g. '87654321'
    /// - On success, responds with [`DescribeOfferingOutput`](crate::operation::describe_offering::DescribeOfferingOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::operation::describe_offering::DescribeOfferingOutput::arn): Unique offering ARN, e.g. 'arn:aws:medialive:us-west-2:123456789012:offering:87654321'
    ///   - [`currency_code(Option<String>)`](crate::operation::describe_offering::DescribeOfferingOutput::currency_code): Currency code for usagePrice and fixedPrice in ISO-4217 format, e.g. 'USD'
    ///   - [`duration(Option<i32>)`](crate::operation::describe_offering::DescribeOfferingOutput::duration): Lease duration, e.g. '12'
    ///   - [`duration_units(Option<OfferingDurationUnits>)`](crate::operation::describe_offering::DescribeOfferingOutput::duration_units): Units for duration, e.g. 'MONTHS'
    ///   - [`fixed_price(Option<f64>)`](crate::operation::describe_offering::DescribeOfferingOutput::fixed_price): One-time charge for each reserved resource, e.g. '0.0' for a NO_UPFRONT offering
    ///   - [`offering_description(Option<String>)`](crate::operation::describe_offering::DescribeOfferingOutput::offering_description): Offering description, e.g. 'HD AVC output at 10-20 Mbps, 30 fps, and standard VQ in US West (Oregon)'
    ///   - [`offering_id(Option<String>)`](crate::operation::describe_offering::DescribeOfferingOutput::offering_id): Unique offering ID, e.g. '87654321'
    ///   - [`offering_type(Option<OfferingType>)`](crate::operation::describe_offering::DescribeOfferingOutput::offering_type): Offering type, e.g. 'NO_UPFRONT'
    ///   - [`region(Option<String>)`](crate::operation::describe_offering::DescribeOfferingOutput::region): AWS region, e.g. 'us-west-2'
    ///   - [`resource_specification(Option<ReservationResourceSpecification>)`](crate::operation::describe_offering::DescribeOfferingOutput::resource_specification): Resource configuration details
    ///   - [`usage_price(Option<f64>)`](crate::operation::describe_offering::DescribeOfferingOutput::usage_price): Recurring usage charge for each reserved resource, e.g. '157.0'
    /// - On failure, responds with [`SdkError<DescribeOfferingError>`](crate::operation::describe_offering::DescribeOfferingError)
    pub fn describe_offering(
        &self,
    ) -> crate::operation::describe_offering::builders::DescribeOfferingFluentBuilder {
        crate::operation::describe_offering::builders::DescribeOfferingFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
