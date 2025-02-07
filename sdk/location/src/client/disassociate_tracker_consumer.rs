// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DisassociateTrackerConsumer`](crate::operation::disassociate_tracker_consumer::builders::DisassociateTrackerConsumerFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`tracker_name(impl ::std::convert::Into<String>)`](crate::operation::disassociate_tracker_consumer::builders::DisassociateTrackerConsumerFluentBuilder::tracker_name) / [`set_tracker_name(Option<String>)`](crate::operation::disassociate_tracker_consumer::builders::DisassociateTrackerConsumerFluentBuilder::set_tracker_name): <p>The name of the tracker resource to be dissociated from the consumer.</p>
    ///   - [`consumer_arn(impl ::std::convert::Into<String>)`](crate::operation::disassociate_tracker_consumer::builders::DisassociateTrackerConsumerFluentBuilder::consumer_arn) / [`set_consumer_arn(Option<String>)`](crate::operation::disassociate_tracker_consumer::builders::DisassociateTrackerConsumerFluentBuilder::set_consumer_arn): <p>The Amazon Resource Name (ARN) for the geofence collection to be disassociated from the tracker resource. Used when you need to specify a resource across all Amazon Web Services. </p>  <ul>   <li> <p>Format example: <code>arn:aws:geo:region:account-id:geofence-collection/ExampleGeofenceCollectionConsumer</code> </p> </li>  </ul>
    /// - On success, responds with [`DisassociateTrackerConsumerOutput`](crate::operation::disassociate_tracker_consumer::DisassociateTrackerConsumerOutput)
    /// - On failure, responds with [`SdkError<DisassociateTrackerConsumerError>`](crate::operation::disassociate_tracker_consumer::DisassociateTrackerConsumerError)
    pub fn disassociate_tracker_consumer(&self) -> crate::operation::disassociate_tracker_consumer::builders::DisassociateTrackerConsumerFluentBuilder{
        crate::operation::disassociate_tracker_consumer::builders::DisassociateTrackerConsumerFluentBuilder::new(self.handle.clone())
    }
}
