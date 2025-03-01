// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListStreams`](crate::operation::list_streams::builders::ListStreamsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`table_name(impl ::std::convert::Into<String>)`](crate::operation::list_streams::builders::ListStreamsFluentBuilder::table_name) / [`set_table_name(Option<String>)`](crate::operation::list_streams::builders::ListStreamsFluentBuilder::set_table_name): <p>If this parameter is provided, then only the streams associated with this table name are returned.</p>
    ///   - [`limit(i32)`](crate::operation::list_streams::builders::ListStreamsFluentBuilder::limit) / [`set_limit(Option<i32>)`](crate::operation::list_streams::builders::ListStreamsFluentBuilder::set_limit): <p>The maximum number of streams to return. The upper limit is 100.</p>
    ///   - [`exclusive_start_stream_arn(impl ::std::convert::Into<String>)`](crate::operation::list_streams::builders::ListStreamsFluentBuilder::exclusive_start_stream_arn) / [`set_exclusive_start_stream_arn(Option<String>)`](crate::operation::list_streams::builders::ListStreamsFluentBuilder::set_exclusive_start_stream_arn): <p>The ARN (Amazon Resource Name) of the first item that this operation will evaluate. Use the value that was returned for <code>LastEvaluatedStreamArn</code> in the previous operation. </p>
    /// - On success, responds with [`ListStreamsOutput`](crate::operation::list_streams::ListStreamsOutput) with field(s):
    ///   - [`streams(Option<Vec<Stream>>)`](crate::operation::list_streams::ListStreamsOutput::streams): <p>A list of stream descriptors associated with the current account and endpoint.</p>
    ///   - [`last_evaluated_stream_arn(Option<String>)`](crate::operation::list_streams::ListStreamsOutput::last_evaluated_stream_arn): <p>The stream ARN of the item where the operation stopped, inclusive of the previous result set. Use this value to start a new operation, excluding this value in the new request.</p>  <p>If <code>LastEvaluatedStreamArn</code> is empty, then the "last page" of results has been processed and there is no more data to be retrieved.</p>  <p>If <code>LastEvaluatedStreamArn</code> is not empty, it does not necessarily mean that there is more data in the result set. The only way to know when you have reached the end of the result set is when <code>LastEvaluatedStreamArn</code> is empty.</p>
    /// - On failure, responds with [`SdkError<ListStreamsError>`](crate::operation::list_streams::ListStreamsError)
    pub fn list_streams(
        &self,
    ) -> crate::operation::list_streams::builders::ListStreamsFluentBuilder {
        crate::operation::list_streams::builders::ListStreamsFluentBuilder::new(self.handle.clone())
    }
}
