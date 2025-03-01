// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteCampaign`](crate::operation::delete_campaign::builders::DeleteCampaignFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`campaign_arn(impl ::std::convert::Into<String>)`](crate::operation::delete_campaign::builders::DeleteCampaignFluentBuilder::campaign_arn) / [`set_campaign_arn(Option<String>)`](crate::operation::delete_campaign::builders::DeleteCampaignFluentBuilder::set_campaign_arn): <p>The Amazon Resource Name (ARN) of the campaign to delete.</p>
    /// - On success, responds with [`DeleteCampaignOutput`](crate::operation::delete_campaign::DeleteCampaignOutput)
    /// - On failure, responds with [`SdkError<DeleteCampaignError>`](crate::operation::delete_campaign::DeleteCampaignError)
    pub fn delete_campaign(
        &self,
    ) -> crate::operation::delete_campaign::builders::DeleteCampaignFluentBuilder {
        crate::operation::delete_campaign::builders::DeleteCampaignFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
