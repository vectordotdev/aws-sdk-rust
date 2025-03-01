// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`RegisterEventTopic`](crate::operation::register_event_topic::builders::RegisterEventTopicFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`directory_id(impl ::std::convert::Into<String>)`](crate::operation::register_event_topic::builders::RegisterEventTopicFluentBuilder::directory_id) / [`set_directory_id(Option<String>)`](crate::operation::register_event_topic::builders::RegisterEventTopicFluentBuilder::set_directory_id): <p>The Directory ID that will publish status messages to the Amazon SNS topic.</p>
    ///   - [`topic_name(impl ::std::convert::Into<String>)`](crate::operation::register_event_topic::builders::RegisterEventTopicFluentBuilder::topic_name) / [`set_topic_name(Option<String>)`](crate::operation::register_event_topic::builders::RegisterEventTopicFluentBuilder::set_topic_name): <p>The Amazon SNS topic name to which the directory will publish status messages. This Amazon SNS topic must be in the same region as the specified Directory ID.</p>
    /// - On success, responds with [`RegisterEventTopicOutput`](crate::operation::register_event_topic::RegisterEventTopicOutput)
    /// - On failure, responds with [`SdkError<RegisterEventTopicError>`](crate::operation::register_event_topic::RegisterEventTopicError)
    pub fn register_event_topic(
        &self,
    ) -> crate::operation::register_event_topic::builders::RegisterEventTopicFluentBuilder {
        crate::operation::register_event_topic::builders::RegisterEventTopicFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
