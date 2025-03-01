// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeThing`](crate::operation::describe_thing::builders::DescribeThingFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`thing_name(impl ::std::convert::Into<String>)`](crate::operation::describe_thing::builders::DescribeThingFluentBuilder::thing_name) / [`set_thing_name(Option<String>)`](crate::operation::describe_thing::builders::DescribeThingFluentBuilder::set_thing_name): <p>The name of the thing.</p>
    /// - On success, responds with [`DescribeThingOutput`](crate::operation::describe_thing::DescribeThingOutput) with field(s):
    ///   - [`default_client_id(Option<String>)`](crate::operation::describe_thing::DescribeThingOutput::default_client_id): <p>The default MQTT client ID. For a typical device, the thing name is also used as the default MQTT client ID. Although we don’t require a mapping between a thing's registry name and its use of MQTT client IDs, certificates, or shadow state, we recommend that you choose a thing name and use it as the MQTT client ID for the registry and the Device Shadow service.</p>  <p>This lets you better organize your IoT fleet without removing the flexibility of the underlying device certificate model or shadows.</p>
    ///   - [`thing_name(Option<String>)`](crate::operation::describe_thing::DescribeThingOutput::thing_name): <p>The name of the thing.</p>
    ///   - [`thing_id(Option<String>)`](crate::operation::describe_thing::DescribeThingOutput::thing_id): <p>The ID of the thing to describe.</p>
    ///   - [`thing_arn(Option<String>)`](crate::operation::describe_thing::DescribeThingOutput::thing_arn): <p>The ARN of the thing to describe.</p>
    ///   - [`thing_type_name(Option<String>)`](crate::operation::describe_thing::DescribeThingOutput::thing_type_name): <p>The thing type name.</p>
    ///   - [`attributes(Option<HashMap<String, String>>)`](crate::operation::describe_thing::DescribeThingOutput::attributes): <p>The thing attributes.</p>
    ///   - [`version(i64)`](crate::operation::describe_thing::DescribeThingOutput::version): <p>The current version of the thing record in the registry.</p> <note>   <p>To avoid unintentional changes to the information in the registry, you can pass the version information in the <code>expectedVersion</code> parameter of the <code>UpdateThing</code> and <code>DeleteThing</code> calls.</p>  </note>
    ///   - [`billing_group_name(Option<String>)`](crate::operation::describe_thing::DescribeThingOutput::billing_group_name): <p>The name of the billing group the thing belongs to.</p>
    /// - On failure, responds with [`SdkError<DescribeThingError>`](crate::operation::describe_thing::DescribeThingError)
    pub fn describe_thing(
        &self,
    ) -> crate::operation::describe_thing::builders::DescribeThingFluentBuilder {
        crate::operation::describe_thing::builders::DescribeThingFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
