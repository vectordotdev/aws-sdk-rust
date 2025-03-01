// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// StartCampaignRequest
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StartCampaignInput {
    /// Identifier representing a Campaign
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
}
impl StartCampaignInput {
    /// Identifier representing a Campaign
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
}
impl StartCampaignInput {
    /// Creates a new builder-style object to manufacture [`StartCampaignInput`](crate::operation::start_campaign::StartCampaignInput).
    pub fn builder() -> crate::operation::start_campaign::builders::StartCampaignInputBuilder {
        crate::operation::start_campaign::builders::StartCampaignInputBuilder::default()
    }
}

/// A builder for [`StartCampaignInput`](crate::operation::start_campaign::StartCampaignInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct StartCampaignInputBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
}
impl StartCampaignInputBuilder {
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
    /// Consumes the builder and constructs a [`StartCampaignInput`](crate::operation::start_campaign::StartCampaignInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::start_campaign::StartCampaignInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::start_campaign::StartCampaignInput {
            id: self.id,
        })
    }
}
