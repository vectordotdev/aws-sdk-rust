// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetMemberDetectors`](crate::operation::get_member_detectors::builders::GetMemberDetectorsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`detector_id(impl ::std::convert::Into<String>)`](crate::operation::get_member_detectors::builders::GetMemberDetectorsFluentBuilder::detector_id) / [`set_detector_id(Option<String>)`](crate::operation::get_member_detectors::builders::GetMemberDetectorsFluentBuilder::set_detector_id): <p>The detector ID for the administrator account.</p>
    ///   - [`account_ids(Vec<String>)`](crate::operation::get_member_detectors::builders::GetMemberDetectorsFluentBuilder::account_ids) / [`set_account_ids(Option<Vec<String>>)`](crate::operation::get_member_detectors::builders::GetMemberDetectorsFluentBuilder::set_account_ids): <p>The account ID of the member account.</p>
    /// - On success, responds with [`GetMemberDetectorsOutput`](crate::operation::get_member_detectors::GetMemberDetectorsOutput) with field(s):
    ///   - [`member_data_source_configurations(Option<Vec<MemberDataSourceConfiguration>>)`](crate::operation::get_member_detectors::GetMemberDetectorsOutput::member_data_source_configurations): <p>An object that describes which data sources are enabled for a member account.</p>
    ///   - [`unprocessed_accounts(Option<Vec<UnprocessedAccount>>)`](crate::operation::get_member_detectors::GetMemberDetectorsOutput::unprocessed_accounts): <p>A list of member account IDs that were unable to be processed along with an explanation for why they were not processed.</p>
    /// - On failure, responds with [`SdkError<GetMemberDetectorsError>`](crate::operation::get_member_detectors::GetMemberDetectorsError)
    pub fn get_member_detectors(
        &self,
    ) -> crate::operation::get_member_detectors::builders::GetMemberDetectorsFluentBuilder {
        crate::operation::get_member_detectors::builders::GetMemberDetectorsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
