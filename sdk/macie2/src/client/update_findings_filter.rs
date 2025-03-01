// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateFindingsFilter`](crate::operation::update_findings_filter::builders::UpdateFindingsFilterFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`action(FindingsFilterAction)`](crate::operation::update_findings_filter::builders::UpdateFindingsFilterFluentBuilder::action) / [`set_action(Option<FindingsFilterAction>)`](crate::operation::update_findings_filter::builders::UpdateFindingsFilterFluentBuilder::set_action): <p>The action to perform on findings that match the filter criteria (findingCriteria). Valid values are: ARCHIVE, suppress (automatically archive) the findings; and, NOOP, don't perform any action on the findings.</p>
    ///   - [`client_token(impl ::std::convert::Into<String>)`](crate::operation::update_findings_filter::builders::UpdateFindingsFilterFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::update_findings_filter::builders::UpdateFindingsFilterFluentBuilder::set_client_token): <p>A unique, case-sensitive token that you provide to ensure the idempotency of the request.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::update_findings_filter::builders::UpdateFindingsFilterFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_findings_filter::builders::UpdateFindingsFilterFluentBuilder::set_description): <p>A custom description of the filter. The description can contain as many as 512 characters.</p>  <p>We strongly recommend that you avoid including any sensitive data in the description of a filter. Other users of your account might be able to see this description, depending on the actions that they're allowed to perform in Amazon Macie.</p>
    ///   - [`finding_criteria(FindingCriteria)`](crate::operation::update_findings_filter::builders::UpdateFindingsFilterFluentBuilder::finding_criteria) / [`set_finding_criteria(Option<FindingCriteria>)`](crate::operation::update_findings_filter::builders::UpdateFindingsFilterFluentBuilder::set_finding_criteria): <p>The criteria to use to filter findings.</p>
    ///   - [`id(impl ::std::convert::Into<String>)`](crate::operation::update_findings_filter::builders::UpdateFindingsFilterFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::update_findings_filter::builders::UpdateFindingsFilterFluentBuilder::set_id): <p>The unique identifier for the Amazon Macie resource that the request applies to.</p>
    ///   - [`name(impl ::std::convert::Into<String>)`](crate::operation::update_findings_filter::builders::UpdateFindingsFilterFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_findings_filter::builders::UpdateFindingsFilterFluentBuilder::set_name): <p>A custom name for the filter. The name must contain at least 3 characters and can contain as many as 64 characters.</p>  <p>We strongly recommend that you avoid including any sensitive data in the name of a filter. Other users of your account might be able to see this name, depending on the actions that they're allowed to perform in Amazon Macie.</p>
    ///   - [`position(i32)`](crate::operation::update_findings_filter::builders::UpdateFindingsFilterFluentBuilder::position) / [`set_position(Option<i32>)`](crate::operation::update_findings_filter::builders::UpdateFindingsFilterFluentBuilder::set_position): <p>The position of the filter in the list of saved filters on the Amazon Macie console. This value also determines the order in which the filter is applied to findings, relative to other filters that are also applied to the findings.</p>
    /// - On success, responds with [`UpdateFindingsFilterOutput`](crate::operation::update_findings_filter::UpdateFindingsFilterOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::operation::update_findings_filter::UpdateFindingsFilterOutput::arn): <p>The Amazon Resource Name (ARN) of the filter that was updated.</p>
    ///   - [`id(Option<String>)`](crate::operation::update_findings_filter::UpdateFindingsFilterOutput::id): <p>The unique identifier for the filter that was updated.</p>
    /// - On failure, responds with [`SdkError<UpdateFindingsFilterError>`](crate::operation::update_findings_filter::UpdateFindingsFilterError)
    pub fn update_findings_filter(
        &self,
    ) -> crate::operation::update_findings_filter::builders::UpdateFindingsFilterFluentBuilder {
        crate::operation::update_findings_filter::builders::UpdateFindingsFilterFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
