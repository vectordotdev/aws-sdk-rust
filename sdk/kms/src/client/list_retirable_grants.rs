// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListRetirableGrants`](crate::operation::list_retirable_grants::builders::ListRetirableGrantsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_retirable_grants::builders::ListRetirableGrantsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`limit(i32)`](crate::operation::list_retirable_grants::builders::ListRetirableGrantsFluentBuilder::limit) / [`set_limit(Option<i32>)`](crate::operation::list_retirable_grants::builders::ListRetirableGrantsFluentBuilder::set_limit): <p>Use this parameter to specify the maximum number of items to return. When this value is present, KMS does not return more than the specified number of items, but it might return fewer.</p>  <p>This value is optional. If you include a value, it must be between 1 and 100, inclusive. If you do not include a value, it defaults to 50.</p>
    ///   - [`marker(impl ::std::convert::Into<String>)`](crate::operation::list_retirable_grants::builders::ListRetirableGrantsFluentBuilder::marker) / [`set_marker(Option<String>)`](crate::operation::list_retirable_grants::builders::ListRetirableGrantsFluentBuilder::set_marker): <p>Use this parameter in a subsequent request after you receive a response with truncated results. Set it to the value of <code>NextMarker</code> from the truncated response you just received.</p>
    ///   - [`retiring_principal(impl ::std::convert::Into<String>)`](crate::operation::list_retirable_grants::builders::ListRetirableGrantsFluentBuilder::retiring_principal) / [`set_retiring_principal(Option<String>)`](crate::operation::list_retirable_grants::builders::ListRetirableGrantsFluentBuilder::set_retiring_principal): <p>The retiring principal for which to list grants. Enter a principal in your Amazon Web Services account.</p>  <p>To specify the retiring principal, use the <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Name (ARN)</a> of an Amazon Web Services principal. Valid principals include Amazon Web Services accounts, IAM users, IAM roles, federated users, and assumed role users. For help with the ARN syntax for a principal, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-arns">IAM ARNs</a> in the <i> <i>Identity and Access Management User Guide</i> </i>.</p>
    /// - On success, responds with [`ListRetirableGrantsOutput`](crate::operation::list_retirable_grants::ListRetirableGrantsOutput) with field(s):
    ///   - [`grants(Option<Vec<GrantListEntry>>)`](crate::operation::list_retirable_grants::ListRetirableGrantsOutput::grants): <p>A list of grants.</p>
    ///   - [`next_marker(Option<String>)`](crate::operation::list_retirable_grants::ListRetirableGrantsOutput::next_marker): <p>When <code>Truncated</code> is true, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent request.</p>
    ///   - [`truncated(bool)`](crate::operation::list_retirable_grants::ListRetirableGrantsOutput::truncated): <p>A flag that indicates whether there are more items in the list. When this value is true, the list in this response is truncated. To get more items, pass the value of the <code>NextMarker</code> element in thisresponse to the <code>Marker</code> parameter in a subsequent request.</p>
    /// - On failure, responds with [`SdkError<ListRetirableGrantsError>`](crate::operation::list_retirable_grants::ListRetirableGrantsError)
    pub fn list_retirable_grants(
        &self,
    ) -> crate::operation::list_retirable_grants::builders::ListRetirableGrantsFluentBuilder {
        crate::operation::list_retirable_grants::builders::ListRetirableGrantsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
