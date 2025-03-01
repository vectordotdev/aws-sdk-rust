// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteQueue`](crate::operation::delete_queue::builders::DeleteQueueFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`queue_url(impl ::std::convert::Into<String>)`](crate::operation::delete_queue::builders::DeleteQueueFluentBuilder::queue_url) / [`set_queue_url(Option<String>)`](crate::operation::delete_queue::builders::DeleteQueueFluentBuilder::set_queue_url): <p>The URL of the Amazon SQS queue to delete.</p>  <p>Queue URLs and names are case-sensitive.</p>
    /// - On success, responds with [`DeleteQueueOutput`](crate::operation::delete_queue::DeleteQueueOutput)
    /// - On failure, responds with [`SdkError<DeleteQueueError>`](crate::operation::delete_queue::DeleteQueueError)
    pub fn delete_queue(
        &self,
    ) -> crate::operation::delete_queue::builders::DeleteQueueFluentBuilder {
        crate::operation::delete_queue::builders::DeleteQueueFluentBuilder::new(self.handle.clone())
    }
}
