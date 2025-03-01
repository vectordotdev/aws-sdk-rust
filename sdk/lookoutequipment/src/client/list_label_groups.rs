// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListLabelGroups`](crate::operation::list_label_groups::builders::ListLabelGroupsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_label_groups::builders::ListLabelGroupsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`label_group_name_begins_with(impl ::std::convert::Into<String>)`](crate::operation::list_label_groups::builders::ListLabelGroupsFluentBuilder::label_group_name_begins_with) / [`set_label_group_name_begins_with(Option<String>)`](crate::operation::list_label_groups::builders::ListLabelGroupsFluentBuilder::set_label_group_name_begins_with): <p> The beginning of the name of the label groups to be listed. </p>
    ///   - [`next_token(impl ::std::convert::Into<String>)`](crate::operation::list_label_groups::builders::ListLabelGroupsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_label_groups::builders::ListLabelGroupsFluentBuilder::set_next_token): <p> An opaque pagination token indicating where to continue the listing of label groups. </p>
    ///   - [`max_results(i32)`](crate::operation::list_label_groups::builders::ListLabelGroupsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_label_groups::builders::ListLabelGroupsFluentBuilder::set_max_results): <p> Specifies the maximum number of label groups to list. </p>
    /// - On success, responds with [`ListLabelGroupsOutput`](crate::operation::list_label_groups::ListLabelGroupsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::list_label_groups::ListLabelGroupsOutput::next_token): <p> An opaque pagination token indicating where to continue the listing of label groups. </p>
    ///   - [`label_group_summaries(Option<Vec<LabelGroupSummary>>)`](crate::operation::list_label_groups::ListLabelGroupsOutput::label_group_summaries): <p> A summary of the label groups. </p>
    /// - On failure, responds with [`SdkError<ListLabelGroupsError>`](crate::operation::list_label_groups::ListLabelGroupsError)
    pub fn list_label_groups(
        &self,
    ) -> crate::operation::list_label_groups::builders::ListLabelGroupsFluentBuilder {
        crate::operation::list_label_groups::builders::ListLabelGroupsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
