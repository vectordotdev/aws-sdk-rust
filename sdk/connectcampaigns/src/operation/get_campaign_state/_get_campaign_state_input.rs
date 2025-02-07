// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// GetCampaignStateRequest
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetCampaignStateInput {
    /// Identifier representing a Campaign
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
}
impl GetCampaignStateInput {
    /// Identifier representing a Campaign
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
}
impl GetCampaignStateInput {
    /// Creates a new builder-style object to manufacture [`GetCampaignStateInput`](crate::operation::get_campaign_state::GetCampaignStateInput).
    pub fn builder() -> crate::operation::get_campaign_state::builders::GetCampaignStateInputBuilder
    {
        crate::operation::get_campaign_state::builders::GetCampaignStateInputBuilder::default()
    }
}

/// A builder for [`GetCampaignStateInput`](crate::operation::get_campaign_state::GetCampaignStateInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetCampaignStateInputBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
}
impl GetCampaignStateInputBuilder {
    /// Identifier representing a Campaign
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// Identifier representing a Campaign
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// Consumes the builder and constructs a [`GetCampaignStateInput`](crate::operation::get_campaign_state::GetCampaignStateInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_campaign_state::GetCampaignStateInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_campaign_state::GetCampaignStateInput { id: self.id },
        )
    }
}
