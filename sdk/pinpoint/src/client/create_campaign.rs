// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateCampaign`](crate::operation::create_campaign::builders::CreateCampaignFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`application_id(impl ::std::convert::Into<String>)`](crate::operation::create_campaign::builders::CreateCampaignFluentBuilder::application_id) / [`set_application_id(Option<String>)`](crate::operation::create_campaign::builders::CreateCampaignFluentBuilder::set_application_id): <p>The unique identifier for the application. This identifier is displayed as the <b>Project ID</b> on the Amazon Pinpoint console.</p>
    ///   - [`write_campaign_request(WriteCampaignRequest)`](crate::operation::create_campaign::builders::CreateCampaignFluentBuilder::write_campaign_request) / [`set_write_campaign_request(Option<WriteCampaignRequest>)`](crate::operation::create_campaign::builders::CreateCampaignFluentBuilder::set_write_campaign_request): <p>Specifies the configuration and other settings for a campaign.</p>
    /// - On success, responds with [`CreateCampaignOutput`](crate::operation::create_campaign::CreateCampaignOutput) with field(s):
    ///   - [`campaign_response(Option<CampaignResponse>)`](crate::operation::create_campaign::CreateCampaignOutput::campaign_response): <p>Provides information about the status, configuration, and other settings for a campaign.</p>
    /// - On failure, responds with [`SdkError<CreateCampaignError>`](crate::operation::create_campaign::CreateCampaignError)
    pub fn create_campaign(
        &self,
    ) -> crate::operation::create_campaign::builders::CreateCampaignFluentBuilder {
        crate::operation::create_campaign::builders::CreateCampaignFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
