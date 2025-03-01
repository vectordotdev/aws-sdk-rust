// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_portfolio_share::_update_portfolio_share_output::UpdatePortfolioShareOutputBuilder;

pub use crate::operation::update_portfolio_share::_update_portfolio_share_input::UpdatePortfolioShareInputBuilder;

/// Fluent builder constructing a request to `UpdatePortfolioShare`.
///
/// <p>Updates the specified portfolio share. You can use this API to enable or disable <code>TagOptions</code> sharing or Principal sharing for an existing portfolio share. </p>
/// <p>The portfolio share cannot be updated if the <code>CreatePortfolioShare</code> operation is <code>IN_PROGRESS</code>, as the share is not available to recipient entities. In this case, you must wait for the portfolio share to be COMPLETED.</p>
/// <p>You must provide the <code>accountId</code> or organization node in the input, but not both.</p>
/// <p>If the portfolio is shared to both an external account and an organization node, and both shares need to be updated, you must invoke <code>UpdatePortfolioShare</code> separately for each share type. </p>
/// <p>This API cannot be used for removing the portfolio share. You must use <code>DeletePortfolioShare</code> API for that action. </p> <note>
/// <p>When you associate a principal with portfolio, a potential privilege escalation path may occur when that portfolio is then shared with other accounts. For a user in a recipient account who is <i>not</i> an Service Catalog Admin, but still has the ability to create Principals (Users/Groups/Roles), that user could create a role that matches a principal name association for the portfolio. Although this user may not know which principal names are associated through Service Catalog, they may be able to guess the user. If this potential escalation path is a concern, then Service Catalog recommends using <code>PrincipalType</code> as <code>IAM</code>. With this configuration, the <code>PrincipalARN</code> must already exist in the recipient account before it can be associated. </p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdatePortfolioShareFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_portfolio_share::builders::UpdatePortfolioShareInputBuilder,
}
impl UpdatePortfolioShareFluentBuilder {
    /// Creates a new `UpdatePortfolioShare`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn customize_middleware(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::update_portfolio_share::UpdatePortfolioShare,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_portfolio_share::UpdatePortfolioShareError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        ::std::result::Result::Ok(crate::client::customize::CustomizableOperation {
            handle,
            operation,
        })
    }

    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn send_middleware(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_portfolio_share::UpdatePortfolioShareOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_portfolio_share::UpdatePortfolioShareError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_portfolio_share::UpdatePortfolioShareOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_portfolio_share::UpdatePortfolioShareError,
        >,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::update_portfolio_share::UpdatePortfolioShare,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::update_portfolio_share::UpdatePortfolioShareError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The language code.</p>
    /// <ul>
    /// <li> <p> <code>jp</code> - Japanese</p> </li>
    /// <li> <p> <code>zh</code> - Chinese</p> </li>
    /// </ul>
    pub fn accept_language(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.accept_language(input.into());
        self
    }
    /// <p>The language code.</p>
    /// <ul>
    /// <li> <p> <code>jp</code> - Japanese</p> </li>
    /// <li> <p> <code>zh</code> - Chinese</p> </li>
    /// </ul>
    pub fn set_accept_language(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_accept_language(input);
        self
    }
    /// <p>The unique identifier of the portfolio for which the share will be updated.</p>
    pub fn portfolio_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.portfolio_id(input.into());
        self
    }
    /// <p>The unique identifier of the portfolio for which the share will be updated.</p>
    pub fn set_portfolio_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_portfolio_id(input);
        self
    }
    /// <p>The Amazon Web Services account Id of the recipient account. This field is required when updating an external account to account type share.</p>
    pub fn account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.account_id(input.into());
        self
    }
    /// <p>The Amazon Web Services account Id of the recipient account. This field is required when updating an external account to account type share.</p>
    pub fn set_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_account_id(input);
        self
    }
    /// <p>Information about the organization node.</p>
    pub fn organization_node(mut self, input: crate::types::OrganizationNode) -> Self {
        self.inner = self.inner.organization_node(input);
        self
    }
    /// <p>Information about the organization node.</p>
    pub fn set_organization_node(
        mut self,
        input: ::std::option::Option<crate::types::OrganizationNode>,
    ) -> Self {
        self.inner = self.inner.set_organization_node(input);
        self
    }
    /// <p>Enables or disables <code>TagOptions</code> sharing for the portfolio share. If this field is not provided, the current state of TagOptions sharing on the portfolio share will not be modified.</p>
    pub fn share_tag_options(mut self, input: bool) -> Self {
        self.inner = self.inner.share_tag_options(input);
        self
    }
    /// <p>Enables or disables <code>TagOptions</code> sharing for the portfolio share. If this field is not provided, the current state of TagOptions sharing on the portfolio share will not be modified.</p>
    pub fn set_share_tag_options(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_share_tag_options(input);
        self
    }
    /// <p>A flag to enables or disables <code>Principals</code> sharing in the portfolio. If this field is not provided, the current state of the <code>Principals</code> sharing on the portfolio share will not be modified. </p>
    pub fn share_principals(mut self, input: bool) -> Self {
        self.inner = self.inner.share_principals(input);
        self
    }
    /// <p>A flag to enables or disables <code>Principals</code> sharing in the portfolio. If this field is not provided, the current state of the <code>Principals</code> sharing on the portfolio share will not be modified. </p>
    pub fn set_share_principals(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_share_principals(input);
        self
    }
}
