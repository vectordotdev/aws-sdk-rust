// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateSlackChannelConfiguration`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_slack_channel_configuration`](crate::client::Client::create_slack_channel_configuration).
///
/// See [`crate::client::fluent_builders::CreateSlackChannelConfiguration`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateSlackChannelConfiguration {
    _private: (),
}
impl CreateSlackChannelConfiguration {
    /// Creates a new builder-style object to manufacture [`CreateSlackChannelConfigurationInput`](crate::input::CreateSlackChannelConfigurationInput).
    pub fn builder() -> crate::input::create_slack_channel_configuration_input::Builder {
        crate::input::create_slack_channel_configuration_input::Builder::default()
    }
    /// Creates a new `CreateSlackChannelConfiguration` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateSlackChannelConfiguration {
    type Output = std::result::Result<
        crate::output::CreateSlackChannelConfigurationOutput,
        crate::error::CreateSlackChannelConfigurationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_slack_channel_configuration_error(response)
        } else {
            crate::operation_deser::parse_create_slack_channel_configuration_response(response)
        }
    }
}

/// Operation shape for `DeleteAccountAlias`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_account_alias`](crate::client::Client::delete_account_alias).
///
/// See [`crate::client::fluent_builders::DeleteAccountAlias`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteAccountAlias {
    _private: (),
}
impl DeleteAccountAlias {
    /// Creates a new builder-style object to manufacture [`DeleteAccountAliasInput`](crate::input::DeleteAccountAliasInput).
    pub fn builder() -> crate::input::delete_account_alias_input::Builder {
        crate::input::delete_account_alias_input::Builder::default()
    }
    /// Creates a new `DeleteAccountAlias` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteAccountAlias {
    type Output = std::result::Result<
        crate::output::DeleteAccountAliasOutput,
        crate::error::DeleteAccountAliasError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_account_alias_error(response)
        } else {
            crate::operation_deser::parse_delete_account_alias_response(response)
        }
    }
}

/// Operation shape for `DeleteSlackChannelConfiguration`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_slack_channel_configuration`](crate::client::Client::delete_slack_channel_configuration).
///
/// See [`crate::client::fluent_builders::DeleteSlackChannelConfiguration`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteSlackChannelConfiguration {
    _private: (),
}
impl DeleteSlackChannelConfiguration {
    /// Creates a new builder-style object to manufacture [`DeleteSlackChannelConfigurationInput`](crate::input::DeleteSlackChannelConfigurationInput).
    pub fn builder() -> crate::input::delete_slack_channel_configuration_input::Builder {
        crate::input::delete_slack_channel_configuration_input::Builder::default()
    }
    /// Creates a new `DeleteSlackChannelConfiguration` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteSlackChannelConfiguration {
    type Output = std::result::Result<
        crate::output::DeleteSlackChannelConfigurationOutput,
        crate::error::DeleteSlackChannelConfigurationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_slack_channel_configuration_error(response)
        } else {
            crate::operation_deser::parse_delete_slack_channel_configuration_response(response)
        }
    }
}

/// Operation shape for `DeleteSlackWorkspaceConfiguration`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_slack_workspace_configuration`](crate::client::Client::delete_slack_workspace_configuration).
///
/// See [`crate::client::fluent_builders::DeleteSlackWorkspaceConfiguration`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteSlackWorkspaceConfiguration {
    _private: (),
}
impl DeleteSlackWorkspaceConfiguration {
    /// Creates a new builder-style object to manufacture [`DeleteSlackWorkspaceConfigurationInput`](crate::input::DeleteSlackWorkspaceConfigurationInput).
    pub fn builder() -> crate::input::delete_slack_workspace_configuration_input::Builder {
        crate::input::delete_slack_workspace_configuration_input::Builder::default()
    }
    /// Creates a new `DeleteSlackWorkspaceConfiguration` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteSlackWorkspaceConfiguration {
    type Output = std::result::Result<
        crate::output::DeleteSlackWorkspaceConfigurationOutput,
        crate::error::DeleteSlackWorkspaceConfigurationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_slack_workspace_configuration_error(response)
        } else {
            crate::operation_deser::parse_delete_slack_workspace_configuration_response(response)
        }
    }
}

/// Operation shape for `GetAccountAlias`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_account_alias`](crate::client::Client::get_account_alias).
///
/// See [`crate::client::fluent_builders::GetAccountAlias`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetAccountAlias {
    _private: (),
}
impl GetAccountAlias {
    /// Creates a new builder-style object to manufacture [`GetAccountAliasInput`](crate::input::GetAccountAliasInput).
    pub fn builder() -> crate::input::get_account_alias_input::Builder {
        crate::input::get_account_alias_input::Builder::default()
    }
    /// Creates a new `GetAccountAlias` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetAccountAlias {
    type Output = std::result::Result<
        crate::output::GetAccountAliasOutput,
        crate::error::GetAccountAliasError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_account_alias_error(response)
        } else {
            crate::operation_deser::parse_get_account_alias_response(response)
        }
    }
}

/// Operation shape for `ListSlackChannelConfigurations`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_slack_channel_configurations`](crate::client::Client::list_slack_channel_configurations).
///
/// See [`crate::client::fluent_builders::ListSlackChannelConfigurations`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListSlackChannelConfigurations {
    _private: (),
}
impl ListSlackChannelConfigurations {
    /// Creates a new builder-style object to manufacture [`ListSlackChannelConfigurationsInput`](crate::input::ListSlackChannelConfigurationsInput).
    pub fn builder() -> crate::input::list_slack_channel_configurations_input::Builder {
        crate::input::list_slack_channel_configurations_input::Builder::default()
    }
    /// Creates a new `ListSlackChannelConfigurations` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListSlackChannelConfigurations {
    type Output = std::result::Result<
        crate::output::ListSlackChannelConfigurationsOutput,
        crate::error::ListSlackChannelConfigurationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_slack_channel_configurations_error(response)
        } else {
            crate::operation_deser::parse_list_slack_channel_configurations_response(response)
        }
    }
}

/// Operation shape for `ListSlackWorkspaceConfigurations`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_slack_workspace_configurations`](crate::client::Client::list_slack_workspace_configurations).
///
/// See [`crate::client::fluent_builders::ListSlackWorkspaceConfigurations`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListSlackWorkspaceConfigurations {
    _private: (),
}
impl ListSlackWorkspaceConfigurations {
    /// Creates a new builder-style object to manufacture [`ListSlackWorkspaceConfigurationsInput`](crate::input::ListSlackWorkspaceConfigurationsInput).
    pub fn builder() -> crate::input::list_slack_workspace_configurations_input::Builder {
        crate::input::list_slack_workspace_configurations_input::Builder::default()
    }
    /// Creates a new `ListSlackWorkspaceConfigurations` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListSlackWorkspaceConfigurations {
    type Output = std::result::Result<
        crate::output::ListSlackWorkspaceConfigurationsOutput,
        crate::error::ListSlackWorkspaceConfigurationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_slack_workspace_configurations_error(response)
        } else {
            crate::operation_deser::parse_list_slack_workspace_configurations_response(response)
        }
    }
}

/// Operation shape for `PutAccountAlias`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`put_account_alias`](crate::client::Client::put_account_alias).
///
/// See [`crate::client::fluent_builders::PutAccountAlias`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PutAccountAlias {
    _private: (),
}
impl PutAccountAlias {
    /// Creates a new builder-style object to manufacture [`PutAccountAliasInput`](crate::input::PutAccountAliasInput).
    pub fn builder() -> crate::input::put_account_alias_input::Builder {
        crate::input::put_account_alias_input::Builder::default()
    }
    /// Creates a new `PutAccountAlias` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutAccountAlias {
    type Output = std::result::Result<
        crate::output::PutAccountAliasOutput,
        crate::error::PutAccountAliasError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_account_alias_error(response)
        } else {
            crate::operation_deser::parse_put_account_alias_response(response)
        }
    }
}

/// Operation shape for `UpdateSlackChannelConfiguration`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_slack_channel_configuration`](crate::client::Client::update_slack_channel_configuration).
///
/// See [`crate::client::fluent_builders::UpdateSlackChannelConfiguration`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateSlackChannelConfiguration {
    _private: (),
}
impl UpdateSlackChannelConfiguration {
    /// Creates a new builder-style object to manufacture [`UpdateSlackChannelConfigurationInput`](crate::input::UpdateSlackChannelConfigurationInput).
    pub fn builder() -> crate::input::update_slack_channel_configuration_input::Builder {
        crate::input::update_slack_channel_configuration_input::Builder::default()
    }
    /// Creates a new `UpdateSlackChannelConfiguration` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateSlackChannelConfiguration {
    type Output = std::result::Result<
        crate::output::UpdateSlackChannelConfigurationOutput,
        crate::error::UpdateSlackChannelConfigurationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_slack_channel_configuration_error(response)
        } else {
            crate::operation_deser::parse_update_slack_channel_configuration_response(response)
        }
    }
}
