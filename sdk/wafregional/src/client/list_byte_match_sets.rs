// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListByteMatchSets`](crate::operation::list_byte_match_sets::builders::ListByteMatchSetsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_marker(impl ::std::convert::Into<String>)`](crate::operation::list_byte_match_sets::builders::ListByteMatchSetsFluentBuilder::next_marker) / [`set_next_marker(Option<String>)`](crate::operation::list_byte_match_sets::builders::ListByteMatchSetsFluentBuilder::set_next_marker): <p>If you specify a value for <code>Limit</code> and you have more <code>ByteMatchSets</code> than the value of <code>Limit</code>, AWS WAF returns a <code>NextMarker</code> value in the response that allows you to list another group of <code>ByteMatchSets</code>. For the second and subsequent <code>ListByteMatchSets</code> requests, specify the value of <code>NextMarker</code> from the previous response to get information about another batch of <code>ByteMatchSets</code>.</p>
    ///   - [`limit(i32)`](crate::operation::list_byte_match_sets::builders::ListByteMatchSetsFluentBuilder::limit) / [`set_limit(i32)`](crate::operation::list_byte_match_sets::builders::ListByteMatchSetsFluentBuilder::set_limit): <p>Specifies the number of <code>ByteMatchSet</code> objects that you want AWS WAF to return for this request. If you have more <code>ByteMatchSets</code> objects than the number you specify for <code>Limit</code>, the response includes a <code>NextMarker</code> value that you can use to get another batch of <code>ByteMatchSet</code> objects.</p>
    /// - On success, responds with [`ListByteMatchSetsOutput`](crate::operation::list_byte_match_sets::ListByteMatchSetsOutput) with field(s):
    ///   - [`next_marker(Option<String>)`](crate::operation::list_byte_match_sets::ListByteMatchSetsOutput::next_marker): <p>If you have more <code>ByteMatchSet</code> objects than the number that you specified for <code>Limit</code> in the request, the response includes a <code>NextMarker</code> value. To list more <code>ByteMatchSet</code> objects, submit another <code>ListByteMatchSets</code> request, and specify the <code>NextMarker</code> value from the response in the <code>NextMarker</code> value in the next request.</p>
    ///   - [`byte_match_sets(Option<Vec<ByteMatchSetSummary>>)`](crate::operation::list_byte_match_sets::ListByteMatchSetsOutput::byte_match_sets): <p>An array of <code>ByteMatchSetSummary</code> objects.</p>
    /// - On failure, responds with [`SdkError<ListByteMatchSetsError>`](crate::operation::list_byte_match_sets::ListByteMatchSetsError)
    pub fn list_byte_match_sets(
        &self,
    ) -> crate::operation::list_byte_match_sets::builders::ListByteMatchSetsFluentBuilder {
        crate::operation::list_byte_match_sets::builders::ListByteMatchSetsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
